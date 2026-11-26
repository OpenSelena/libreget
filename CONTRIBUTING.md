# Contributing to LibreGet

- Questions and help: [Discussions](https://github.com/OpenSelena/libreget/discussions) or [Telegram](https://t.me/OpenSelena)
- Bugs and feature requests: [Issues](https://github.com/OpenSelena/libreget/issues/new/choose)
- Security vulnerabilities: [SECURITY.md](SECURITY.md) (do not open a public issue)

## Before writing code

Read [GLOSSARY.md](GLOSSARY.md), [DESIGN.md](DESIGN.md), and [docs/adr/](docs/adr) for project naming and UI tokens. Comment on an issue before working on it to avoid duplicate PRs, and open a [discussion](https://github.com/OpenSelena/libreget/discussions) before starting large features or plugin changes.

UI translations are handled on [Weblate](https://hosted.weblate.org/engage/libreget/) in the browser without a local build.

## Setup

| Tool | Requirement | Notes |
|---|---|---|
| [Rust](https://rustup.rs/) | `1.97.0` | Pinned in [rust-toolchain.toml](rust-toolchain.toml) (`rustfmt`, `clippy`) because plugin ABIs must match the host |
| [Node.js](https://nodejs.org/) | `18+` | LTS recommended |
| [pnpm](https://pnpm.io/) | `10.29.3` | Set via `packageManager` in [package.json](package.json) (`corepack enable` picks it up) |

On Linux, install the Tauri/WebKit system packages first (matching [README.md](README.md#build-from-source) and [.github/workflows/ci.yml](.github/workflows/ci.yml)):

```bash
sudo apt-get update && sudo apt-get install -y \
  libwebkit2gtk-4.1-dev build-essential curl wget file libxdo-dev libssl-dev \
  libayatana-appindicator3-dev librsvg2-dev patchelf libasound2-dev \
  libpipewire-0.3-dev clang libclang-dev
```

Clone and start the app:

```bash
git clone https://github.com/OpenSelena/libreget.git
cd libreget
pnpm install
pnpm tauri dev
```

Use `pnpm tauri dev` for full app work or `pnpm dev` when you only need the Vite frontend. Running `pnpm prepare` points `core.hooksPath` at [.githooks/](.githooks), which formats staged Rust files on commit and checks `cargo fmt` before push.

## Where things live

- `src/` holds the Svelte 5 / SvelteKit frontend (`src/routes/`, `src/components/`, `src/lib/`).
- `src-tauri/src/` contains Tauri host commands, the local bridge (`127.0.0.1:47720–47729`), the MCP server, plugin loading, tray, and platform code.
- `src-tauri/libreget-core/` has shared models, path helpers, platform extractors, and download logic.
- `src-tauri/libreget-cli/` is the standalone `libreget-cli` binary.
- `src-tauri/libreget-plugin-sdk/` and `src-tauri/plugins/` hold the plugin SDK and embedded core plugins (`convert`, `telegram`, `courses`, `grammers-session`).
- `browser-extension/` contains the Chrome (`chrome/`) and Firefox (`firefox/`) extensions.
- `scripts/` contains build, version-bump, i18n, UI lint, contrast audit, accessibility audit, and smoke-test scripts.

## Branches

Open pull requests against **`dev`**, not `main`. Pushes and PRs on `dev` run GitHub Actions CI on Linux (Ubuntu and Debian Bookworm), Windows, and macOS. Maintainers merge `dev` into `main` once CI is green.

## Checks

Run the checks for the parts you touched before pushing; CI runs the same gates:

- Frontend: `pnpm check` (`svelte-kit sync` + `svelte-check`), `pnpm test` (Vitest), `pnpm check:i18n`, and `pnpm lint:ui <target>` whenever you edit Svelte routes (`src/routes/`), UI components (`src/components/`), or CSS (`src/app.css`, `src/lib/style/`).
- Rust: `pnpm fmt:rs` / `pnpm check:fmt`, `cargo clippy --workspace --all-targets` and `cargo test --workspace` inside `src-tauri/`, and `pnpm check:clippy-baseline` from the root. Fix any Clippy warning you introduce—never run `pnpm update:clippy-baseline` to hide your own warnings.
- Browser extension: edit files under `browser-extension/chrome/`, then run `pnpm extension:sync` to sync shared files, `_locales`, and `icons` into `browser-extension/firefox/` (`manifest.json` stays per-browser).

## Rules that save review rounds

Add new interface strings in `src/lib/i18n/en.json` only, then run `pnpm generate:i18n-keys` and commit the updated `src/lib/i18n/keys.ts`. Translations for the other 11 locales (`ru`, `el`, `pt`, `zh`, `zh-TW`, `ja`, `it`, `fr`, `es`, `fa`, `lo`) come from [Weblate](https://hosted.weblate.org/engage/libreget/), so don't commit machine translations in feature PRs.

When you change documented user-facing behavior in [README.md](README.md), update [README.ja.md](README.ja.md), [README_pt_br.md](README_pt_br.md), [README.ru.md](README.ru.md), and [README_zh_CN.md](README_zh_CN.md) to match (or flag what needs translating). Keep scratch notes, temp scripts, and personal files out of git.

## Pull requests and AI tools

Write commit messages and PR descriptions in English ([Conventional Commits](https://www.conventionalcommits.org/) like `feat(scope): ...`, `fix: ...`, `docs: ...` is preferred), keep each PR focused on one change, and fill out the PR template with what changed for users and how you tested it. OpenSelena maintainers (`igect`, `ebnycuhie`) review and merge pull requests, and everyone taking part follows the [Code of Conduct](CODE_OF_CONDUCT.md).

You can use AI tools when writing code or tests, as long as you read every line you send, ran the checks and tested the change in the running app yourself, tick the AI disclosure box in the PR template, and keep the diff small. Bug reports and security reports must also come from behavior you actually reproduced, not raw guesses from reading code.
