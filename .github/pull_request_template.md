## What changes for the user

<!-- Describe what changes for LibreGet users or developers. Link the related issue if one exists (e.g., Fixes #123). -->

## How it was tested

<!-- Mention the OS/environment, manual steps verified, and automated tests added or run. -->

## Checklist

- [ ] Targets the `dev` branch (unless a maintainer requested otherwise)
- [ ] Frontend checks pass (`pnpm check`, `pnpm test`, `pnpm check:i18n`)
- [ ] UI verification passed (`pnpm lint:ui <target>` if routes, components, or CSS changed)
- [ ] Rust checks pass (`pnpm check:fmt`, `cargo clippy --workspace --all-targets`, `pnpm check:clippy-baseline`, `cargo test --workspace` if Rust code changed)
- [ ] New UI text was added only to `src/lib/i18n/en.json` and `src/lib/i18n/keys.ts` was regenerated (`pnpm generate:i18n-keys`)
- [ ] Browser extension changes under `browser-extension/chrome/` were synced via `pnpm extension:sync` (if applicable)
- [ ] User-facing documentation (`README*.md`) was updated (if applicable)
- [ ] An AI tool wrote a meaningful part of this change, and I reviewed, ran, and can explain every line (leave unchecked if not applicable)

See [CONTRIBUTING.md](CONTRIBUTING.md) for details.
