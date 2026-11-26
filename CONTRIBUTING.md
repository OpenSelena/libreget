# Contributing to LibreGet

Thanks for contributing to LibreGet. This guide explains how the repository is organized, how to run it locally, which checks CI enforces, and how pull requests are reviewed.

For questions and quick help, use [Discussions](https://github.com/OpenSelena/libreget/discussions) or [Telegram](https://t.me/OpenSelena). For bugs and feature proposals, use [Issues](https://github.com/OpenSelena/libreget/issues/new/choose). For security issues, follow [SECURITY.md](SECURITY.md) and do not open a public issue.

## Contents

- [Where to start](#where-to-start)
- [Prerequisites and setup](#prerequisites-and-setup)
- [Repository structure](#repository-structure)
- [Branch workflow](#branch-workflow)
- [Checks and verification](#checks-and-verification)
- [Project rules](#project-rules)
- [Pull requests and commits](#pull-requests-and-commits)
- [AI-assisted contributions](#ai-assisted-contributions)

## Where to start

- **Project docs**: Read [GLOSSARY.md](GLOSSARY.md), [DESIGN.md](DESIGN.md), and [docs/adr/](docs/adr) to understand domain terms, UI tokens, and architectural decisions.
- **Pick an issue**: Look for open issues in [Issues](https://github.com/OpenSelena/libreget/issues). Comment on an issue before starting work so efforts do not overlap.
- **Translate the UI**: Translations are managed on [Weblate](https://hosted.weblate.org/engage/libreget/) in the browser without a local build.
- **Large changes**: Open a [discussion](https://github.com/OpenSelena/libreget/discussions) or feature request before writing significant new features or plugin changes.

## Prerequisites and setup

| Tool | Requirement | Source / Notes |
|---|---|---|
| [Rust](https://rustup.rs/) | `1.97.0` | Pinned in [rust-toolchain.toml](rust-toolchain.toml) (`rustfmt`, `clippy`) for plugin ABI compatibility |
| [Node.js](https://nodejs.org/) | `18+` | LTS recommended |
| [pnpm](https://pnpm.io/) | `10.29.3` | Declared via `packageManager` in [package.json](package.json) (`corepack enable` picks it up) |

On Linux, install the system packages required by Tauri/WebKit (shown in [README.md](README.md#build-from-source) and [.github/workflows/ci.yml](.github/workflows/ci.yml)):

```bash
sudo apt-get update && sudo apt-get install -y \
  libwebkit2gtk-4.1-dev build-essential curl wget file libxdo-dev libssl-dev \
  libayatana-appindicator3-dev librsvg2-dev patchelf libasound2-dev \
  libpipewire-0.3-dev clang libclang-dev
```

Clone and run locally:

```bash
git clone https://github.com/OpenSelena/libreget.git
cd libreget
pnpm install
pnpm tauri dev
```

- `pnpm tauri dev` starts the Vite dev server and builds the Rust Tauri backend.
- `pnpm dev` starts only the frontend in Vite (useful when working purely on UI or mockable frontend code).
- `pnpm prepare` configures `core.hooksPath` to [.githooks/](.githooks), which runs `cargo fmt` on staged Rust files on commit and verifies `cargo fmt --check` before push.

## Repository structure

- `src/` — Svelte 5 / SvelteKit frontend (`src/routes/`, `src/components/`, `src/lib/`).
- `src-tauri/` — Tauri v2 desktop backend and Rust workspace (`src-tauri/Cargo.toml`):
  - `src-tauri/src/` — Tauri host commands, local bridge (`127.0.0.1:47720–47729`), MCP server, plugin host/loader, tray, and platform code.
  - `src-tauri/libreget-core/` — Shared core models, path helpers, platform extractors, and download logic.
  - `src-tauri/libreget-cli/` — Standalone `libreget-cli` binary.
  - `src-tauri/libreget-plugin-sdk/` — Plugin trait SDK.
  - `src-tauri/plugins/` — Embedded core plugins (`convert`, `telegram`, `courses`, `grammers-session`).
- `browser-extension/` — Chrome (`chrome/`) and Firefox (`firefox/`) companion extensions.
- `scripts/` — Build, version bump, i18n key generation, UI lint, contrast audit, accessibility audit, and smoke-test scripts.

## Branch workflow

- Target the **`dev`** branch for pull requests. All feature development, refactoring, and bug fixes land on `dev` first.
- Pushing or targeting `dev` triggers GitHub Actions CI across Linux (Ubuntu and Debian Bookworm), Windows, and macOS.
- **`main`** is reserved for verified releases and is merged from `dev` by maintainers after CI passes.

## Checks and verification

Run the relevant checks locally before pushing. CI runs these on every pull request:

### Frontend

- `pnpm check` — Runs `svelte-kit sync` and `svelte-check` (TypeScript + Svelte template/a11y diagnostics).
- `pnpm test` — Runs the Vitest unit test suite.
- `pnpm check:i18n` — Verifies all locale files stay in sync with `src/lib/i18n/en.json`.
- `pnpm generate:i18n-keys` — Regenerates `src/lib/i18n/keys.ts` after changing keys in `en.json` (commit the updated `keys.ts`).
- `pnpm lint:ui <target>` — Required whenever modifying Svelte routes (`src/routes/`), UI components (`src/components/`), or CSS (`src/app.css`, `src/lib/style/`).

### Rust (from repo root or `src-tauri/`)

- `pnpm fmt:rs` / `pnpm check:fmt` — Formats or checks formatting across the Rust workspace.
- `cargo clippy --workspace --all-targets` (inside `src-tauri/`) — Runs Clippy across workspace crates.
- `pnpm check:clippy-baseline` — Fails on any newly introduced Clippy warnings compared to the committed baselines. Fix warnings in your code; never run `pnpm update:clippy-baseline` to mask your own warnings.
- `cargo test --workspace` (inside `src-tauri/`) — Runs Rust workspace unit and integration tests.

### Browser extension

- `pnpm extension:sync` — Copies shared scripts, styles, popup/options pages, `_locales`, and `icons` from `browser-extension/chrome/` to `browser-extension/firefox/`.

## Project rules

- **Localization**: Add or update interface text in `src/lib/i18n/en.json` only, then run `pnpm generate:i18n-keys` and commit `src/lib/i18n/keys.ts`. Other locales (`ru`, `el`, `pt`, `zh`, `zh-TW`, `ja`, `it`, `fr`, `es`, `fa`, `lo`) come from [Weblate](https://hosted.weblate.org/engage/libreget/); do not add machine translations in feature PRs.
- **Browser extension edits**: Edit files under `browser-extension/chrome/` (except browser-specific `manifest.json`), then run `pnpm extension:sync`.
- **README translations**: When changing documented user-facing behavior in [README.md](README.md), update or note corresponding sections in [README.ja.md](README.ja.md), [README_pt_br.md](README_pt_br.md), [README.ru.md](README.ru.md), and [README_zh_CN.md](README_zh_CN.md).
- **Comments and UI standards**: Keep code comments focused on non-obvious invariants or rationale; follow the design tokens and patterns in [DESIGN.md](DESIGN.md).
- **Clean working tree**: Do not commit personal scratch notes, temporary files, or local editor artifacts.

## Pull requests and commits

- Write commit messages and PR descriptions in English. [Conventional Commits](https://www.conventionalcommits.org/) style (`feat(scope): ...`, `fix: ...`, `docs: ...`, `refactor: ...`, `chore: ...`) is preferred.
- Keep pull requests focused on one change at a time, and fill out the pull request template with what changed for users and how you tested it.
- OpenSelena maintainers (`igect`, `ebnycuhie`) review and merge pull requests after CI passes.
- Everyone participating in issues, pull requests, translations, and chats follows the [Code of Conduct](CODE_OF_CONDUCT.md).

## AI-assisted contributions

You may use AI coding tools when contributing to LibreGet, provided you take full responsibility for what you submit:

- **You own the diff**: Read every line you submit, understand how it works, and be ready to answer review questions yourself.
- **You tested it locally**: Run `pnpm test`, `pnpm check`, `pnpm lint:ui` (if UI changed), and Rust checks as applicable, and verify the behavior in the running app. Untested PRs may be closed.
- **Disclose AI use**: Tick the AI-assistance disclosure item in the pull request template and note what you verified manually.
- **Keep PRs scoped**: Avoid large, unrequested generated diffs.
- **Issue and security reports**: Bug and security reports must describe reproducible behavior you directly observed. Unverified reports generated solely by reading code will be closed.
