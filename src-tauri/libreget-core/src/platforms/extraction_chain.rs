use std::future::Future;
use std::time::{Duration, Instant};

use anyhow::{Result, anyhow};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Failure {
    RateLimited,
    Authorization,
    Missing,
    Timeout,
    Other,
}

impl Failure {
    pub(crate) fn classify(error: &anyhow::Error) -> Self {
        if let Some(status) = error
            .downcast_ref::<reqwest::Error>()
            .and_then(reqwest::Error::status)
        {
            return match status.as_u16() {
                429 => Self::RateLimited,
                401 | 403 => Self::Authorization,
                404 | 410 => Self::Missing,
                _ => Self::Other,
            };
        }
        let reason = format!("{error:#}").to_ascii_lowercase();
        if [
            "http error 429",
            "429 too many requests",
            "please wait a few minutes",
        ]
        .iter()
        .any(|p| reason.contains(p))
        {
            Self::RateLimited
        } else if [
            "http error 401",
            "401 unauthorized",
            "private post",
            "private account",
            "captcha",
            "checkpoint_required",
        ]
        .iter()
        .any(|p| reason.contains(p))
            || (reason.contains("login required")
                && !reason.contains("rate-limit reached or login required"))
        {
            Self::Authorization
        } else if reason.contains("timeout") || reason.contains("timed out") {
            Self::Timeout
        } else {
            Self::Other
        }
    }

    pub(crate) fn terminal(self) -> bool {
        matches!(self, Self::Authorization | Self::RateLimited)
    }
}

pub(crate) struct ExtractionChain {
    started: Instant,
    attempts: Vec<(&'static str, Failure)>,
}

impl ExtractionChain {
    pub(crate) fn new() -> Self {
        Self {
            started: Instant::now(),
            attempts: Vec::new(),
        }
    }

    pub(crate) async fn attempt<T>(
        &mut self,
        method: &'static str,
        operation: impl Future<Output = Result<T>>,
    ) -> Option<T> {
        if self
            .attempts
            .last()
            .is_some_and(|(_, failure)| failure.terminal())
        {
            return None;
        }
        let start = Instant::now();
        let budget = Duration::from_secs(42).saturating_sub(self.started.elapsed());
        if budget.is_zero() {
            return None;
        }
        let outcome = tokio::time::timeout(budget.min(Duration::from_secs(14)), operation).await;
        let failure = match outcome {
            Ok(Ok(value)) => {
                tracing::info!(
                    method,
                    elapsed_ms = start.elapsed().as_millis(),
                    total_ms = self.started.elapsed().as_millis(),
                    "media extraction succeeded"
                );
                return Some(value);
            }
            Ok(Err(error)) => Failure::classify(&error),
            Err(_) => Failure::Timeout,
        };
        tracing::info!(
            method,
            ?failure,
            terminal = failure.terminal(),
            elapsed_ms = start.elapsed().as_millis(),
            "media extraction attempt failed"
        );
        self.attempts.push((method, failure));
        None
    }

    pub(crate) fn error(self) -> anyhow::Error {
        match self.attempts.last().map(|(_, reason)| reason) {
            Some(Failure::Authorization) => anyhow!(
                "Instagram requires authorized access. Check your session and access to the post."
            ),
            Some(Failure::RateLimited) => anyhow!("Instagram returned HTTP 429. Try again later."),
            _ => anyhow!("Instagram media extraction failed: {:?}", self.attempts),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn recoverable_error_tries_next_method_but_rate_limit_does_not() {
        let mut chain = ExtractionChain::new();
        assert!(
            chain
                .attempt::<()>("one", async { Err(anyhow!("empty")) })
                .await
                .is_none()
        );
        assert_eq!(chain.attempt("two", async { Ok(42) }).await, Some(42));
        assert!(
            chain
                .attempt::<()>("three", async { Err(anyhow!("HTTP Error 429")) })
                .await
                .is_none()
        );
        assert!(
            chain
                .attempt::<()>("four", async { panic!("must not poll") })
                .await
                .is_none()
        );
        assert!(chain.error().to_string().contains("429"));
    }
}
