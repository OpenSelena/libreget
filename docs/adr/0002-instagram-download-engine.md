# Instagram download engine

## Audit and routing (2026-11-22)

The CLI and Tauri application register the same Rust core Instagram downloader.
Previously it used yt-dlp first for every URL, could run it a second time after
the native routes failed, and did not try GraphQL when embed returned valid JSON
but unusable media. Stories were rejected before resolution. Carousel progress
could block on an undrained channel, and a late failure could restart the full
post with a video-only extractor after some items were already written.

| Project and observed release | Role and integration decision |
| --- | --- |
| [LibreGet v0.16.6](https://github.com/OpenSelena/libreget/releases/tag/v0.16.6), GPL-3.0 | Reuse the Rust registry, HTTP downloader, configured sessions and existing output types. |
| [yt-dlp 2026.10.26](https://github.com/yt-dlp/yt-dlp/releases/tag/2026.10.26), Unlicense source | Keep for video formats, FFmpeg, HLS/DASH and existing cookie settings; not an image-album engine. |
| [gallery-dl v1.32.21](https://github.com/mikf/gallery-dl/releases/tag/v1.32.21), GPL-2.0 | Use the already supported optional CLI for photo/mixed posts and individual Stories; JSON metadata only. |
| [Instaloader v4.16](https://github.com/instaloader/instaloader/releases/tag/v4.16), MIT | Instagram-specialized Python/session workflow, but overlaps with gallery-dl and adds another runtime/session model. |
| [Cobalt](https://github.com/imputnet/cobalt), AGPL-3.0 | Separate service and API/picker; not a default dependency because of deployment and URL disclosure. |
| [Streamlink 8.6.3](https://github.com/streamlink/streamlink/releases/tag/8.6.3), BSD-2-Clause | Useful for live streams and manifests, not an Instagram post/album extractor. |
| [Lux v0.24.1](https://github.com/iawia002/lux), MIT; [you-get v0.4.1743](https://github.com/soimort/you-get) | General-purpose CLI alternatives; reviewed, but no demonstrated advantage for this failure set. |
| [instagrapi 3.2.19](https://github.com/subzeroid/instagrapi) | Active specialized client, but a new unofficial authenticated API/session surface was not justified. |

Upstream evidence: [yt-dlp image-only carousel issue](https://github.com/yt-dlp/yt-dlp/issues/17077)
and [gallery-dl reports of Instagram 429 and redirects](https://github.com/mikf/gallery-dl/issues/9777).
These are observable failure modes, not comparable success rates. No controlled
multi-account benchmark was available; no speed or success percentage is claimed.

## Operational behavior

- Reels: yt-dlp, then local gallery-dl, then the existing embed and native
  metadata paths. A cover image cannot satisfy a Reel.
- Posts: gallery-dl, embed, native metadata, then yt-dlp video compatibility.
  Mixed gallery-dl results preserve order and media type; they use the existing
  direct downloader with length checks, resume/range support and no transcoding.
- Individual Stories: gallery-dl only, using the optional configured local
  Instagram cookie file. Profile-wide Stories are not enumerated. Authorization
  and availability remain subject to Instagram.
- Install gallery-dl on PATH or in LibreGet's tools directory to enable that
  method. The adapter does not fetch a binary during an extraction attempt.
  It ignores external CLI config and disables cookie persistence in gallery-dl;
  access to private content requires a user-authorized session.
- The extraction chain has a total 42-second budget (up to 14 seconds per
  method), plus at most 10 seconds for explicit share resolution. No method is
  run after success. Explicit authorization failures and HTTP 429 stop the
  chain. Attempt logs contain method, duration and failure class, not input
  URLs, CDN signatures, stderr, or cookie values.
- An expired direct URL for a single item can be refreshed once from the same
  post. Carousel failures do not restart the entire post with yt-dlp. Existing
  direct transfer retries handle interrupted connections; a 429 is not retried.
- The optional public check is
  `cargo test -p libreget-core public_instagram_extraction_live -- --ignored --nocapture`
  from `src-tauri`, with `LIBREGET_PUBLIC_INSTAGRAM_URL` set to a known public post.
  Set `LIBREGET_PUBLIC_INSTAGRAM_DOWNLOAD=1` to also test download to a temporary
  directory. This does not test authenticated access. Do not commit cookies.

New adapters should return a validated media result before the chain marks the
attempt successful. Choose ordering per content type and only promote a method
with representative measurements; neither cross-session caching of cookies nor
automatic switching of accounts/proxies is part of this engine.

## Verification

On Windows, the core suite passed 1,293 tests (59 ignored). The public Reel
`Df8Fmx3NuQP` resolved three quality entries and produced a 1,526,542-byte
file through LibreGet. The carousel `Df7U0inFUE9` was not downloadable from
this environment without a session: gallery-dl returned a JSON HTTP 401
error record even though the process exited with code 0. The engine reported
authorization failure. Mixed carousel parsing is covered by fixtures, not a
successful live album download. Authenticated Stories and comparative
throughput, success-rate and latency measurements were not verified.
