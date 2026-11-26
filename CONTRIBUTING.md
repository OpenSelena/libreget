# Contributing to LibreGet

- Questions and help: [Discussions](https://github.com/OpenSelena/libreget/discussions) or [Telegram](https://t.me/OpenSelena)
- Bugs and feature requests: [Issues](https://github.com/OpenSelena/libreget/issues/new/choose)
- Security vulnerabilities: [SECURITY.md](SECURITY.md) (do not open a public issue)

## Before writing code

Read [GLOSSARY.md](GLOSSARY.md), [DESIGN.md](DESIGN.md), and [docs/adr/](docs/adr) for canonical naming, UI tokens, and existing architecture decisions. Comment on an open issue before starting work so two people do not solve the same thing, and open a [discussion](https://github.com/OpenSelena/libreget/discussions) before implementing large features or new plugins.

LibreGet does not accept changes that bypass DRM, break paywalls, or expose user credentials. UI translations are managed on [Weblate](https://hosted.weblate.org/engage/libreget/)—do not submit machine-translated locale files in feature PRs.

## Setup

| Tool | Version | Notes |
| :---: | --- | --- |
| <a href="https://rustup.rs/"><img src="https://cdn.simpleicons.org/rust/CE422B" width="24" height="24" alt="Rust" title="Rust" /></a> | `1.97.0` | Pinned in [rust-toolchain.toml](rust-toolchain.toml) (`rustfmt` + `clippy`) so host and plugin ABIs match |
| <a href="https://nodejs.org/"><img src="https://api.iconify.design/logos/nodejs-icon.svg" width="24" height="24" alt="Node.js" title="Node.js" /></a> | `18+` (LTS) | CI runs `lts/*` in [.github/workflows/ci.yml](.github/workflows/ci.yml) |
| <a href="https://pnpm.io/"><img src="https://cdn.jsdelivr.net/gh/devicons/devicon@latest/icons/pnpm/pnpm-original.svg" width="24" height="24" alt="pnpm" title="pnpm" /></a> | `10.29.3` | Set via `packageManager` in [package.json](package.json); use `corepack enable` |
| <a href="https://www.npmjs.com/"><img src="https://api.iconify.design/logos/npm.svg" height="13" alt="npm" title="npm" /></a> | Bundled with Node | Fallback to install `pnpm@10.29.3` globally (`npm i -g pnpm@10.29.3`); do not use `npm install` in the repo (`pnpm-lock.yaml` is authoritative) |

On Linux, install the WebKit/Tauri system libraries first (same packages CI installs in `debian:bookworm` and `ubuntu-latest`):

```bash
sudo apt-get update && sudo apt-get install -y \
  libwebkit2gtk-4.1-dev build-essential curl wget file libxdo-dev libssl-dev \
  libayatana-appindicator3-dev librsvg2-dev patchelf libasound2-dev \
  libpipewire-0.3-dev clang libclang-dev
```

Clone, install dependencies, and run:

```bash
git clone https://github.com/OpenSelena/libreget.git
cd libreget
pnpm install
pnpm tauri dev
```

- `pnpm tauri dev` runs the Vite dev server on `http://localhost:1420` alongside the Tauri Rust backend (`src-tauri/tauri.conf.json`).
- `pnpm dev` runs only the SvelteKit/Vite frontend when iterating on UI or Vitest-covered modules without compiling Rust.
- `pnpm tauri build --config '{"bundle":{"createUpdaterArtifacts":false}}'` builds a release binary locally without requiring the maintainer's `TAURI_SIGNING_PRIVATE_KEY`.
- `pnpm prepare` configures `core.hooksPath` to [.githooks/](.githooks) (`pre-commit` auto-formats staged `.rs` files; `pre-push` checks `cargo fmt`). On Linux/macOS, ensure both hook files are executable (`chmod +x .githooks/pre-commit .githooks/pre-push`) or run `pnpm fmt:rs` manually before committing.

## Codebase layout

- `src/routes/`, `src/components/`, `src/lib/` — Svelte 5 / SvelteKit UI (`src/app.css` and `src/lib/style/` hold design tokens and stylesheets).
- `src-tauri/src/` — Tauri v2 host, `commands/`, local bridge on `127.0.0.1:47720–47729` (`local_bridge.rs`), MCP server (`mcp.rs`), plugin host/loader, and tray. New Tauri commands must be registered in `.invoke_handler(tauri::generate_handler![...])` in [src-tauri/src/lib.rs](src-tauri/src/lib.rs) and permitted in [src-tauri/capabilities/default.json](src-tauri/capabilities/default.json).
- `src-tauri/libreget-core/` — Core models, config/path helpers, and native platform extractors in `src-tauri/libreget-core/src/platforms/`.
- `src-tauri/libreget-cli/` — Standalone `libreget-cli` binary.
- `src-tauri/libreget-plugin-sdk/` & `src-tauri/plugins/` — Plugin SDK trait and embedded plugins (`convert`, `telegram`, `courses`, `grammers-session`).
- `browser-extension/` — Chrome (`chrome/`) and Firefox (`firefox/`) extensions. Edit shared scripts/styles/pages/`_locales`/`icons` in `chrome/` and sync to `firefox/` with `pnpm extension:sync` (`manifest.json` stays per-browser).
- `scripts/` — Automation for UI lint (`lint-ui.mjs`), Clippy baseline (`clippy-baseline.mjs`), i18n keys (`generate-i18n-keys.js`), smoke test (`smoke-test.mjs`), contrast/a11y audits (`contrast-audit.mjs`, `a11y-audit.mjs`), and version bumps (`bump-version.js`).

## Branch workflow

Target **`dev`** for all pull requests. Pushes and PRs against `dev` run GitHub Actions CI across Ubuntu, Debian Bookworm, Windows, and macOS. Maintainers merge `dev` into `main` only after CI passes.

## Local checks

Run the checks for the areas you changed before pushing:

### Frontend and i18n

```bash
pnpm check                 # svelte-kit sync + svelte-check
pnpm test                  # Vitest suite
pnpm check:i18n            # checks all 11 other locales stay in sync with src/lib/i18n/en.json
```

- When adding or renaming translation keys in `src/lib/i18n/en.json`, run `pnpm generate:i18n-keys` and commit the updated `src/lib/i18n/keys.ts` (CI fails `git diff --exit-code -- src/lib/i18n/keys.ts` if stale).
- Whenever you modify Svelte routes (`src/routes/`), components (`src/components/`), or CSS (`src/app.css`, `src/lib/style/`), run targeted UI lint ([.agents/skills/ui-lint/SKILL.md](.agents/skills/ui-lint/SKILL.md)):

```bash
pnpm lint:ui /settings               # single route (light + dark)
pnpm lint:ui --shell                 # layout, shell components, shared button/primitive styles
pnpm lint:ui /settings --all-themes  # all 14 themes in src/app.css
```

### Rust

```bash
pnpm check:fmt                       # or pnpm fmt:rs to format
pnpm check:clippy-baseline           # checks against src-tauri/clippy-baseline.<platform>.json
```

Inside `src-tauri/`:

```bash
cargo clippy --workspace --all-targets
cargo test --workspace
```

- On Windows, standalone test binaries linking Tauri/WebView without an embedded manifest hit `0xc0000139`, so run `cargo test --workspace --exclude libreget-plugin-courses` (matches CI).
- Fix any new Clippy warning you introduce; do not run `pnpm update:clippy-baseline` to silence new warnings.

### Smoke test and Flatpak metainfo

```bash
node scripts/smoke-test.mjs src-tauri/target/debug/libreget --portable
appstreamcli validate flatpak/com.openselena.libreget.metainfo.xml
```

## Runtime environment variables

| Variable | Effect |
|---|---|
| `LIBREGET_DATA_DIR` | Override the profile/data directory (used for test/dev isolation and `--portable` smoke runs) |
| `LIBREGET_PORTABLE=1` | Enable portable mode alongside `LIBREGET_DATA_DIR` (enabled automatically if `portable.txt` sits beside the executable) |
| `LIBREGET_RELAY` | Override the P2P transfer relay server address |
| `LIBREGET_I18N_STRICT=0` | Disable strict locale validation during UI/a11y dev scripts |
| `LIBREGET_SMOKE_EXIT_MS` | Auto-exit timeout in milliseconds for `scripts/smoke-test.mjs` |
| `LIBREGET_SKIP_PLUGIN_DEPLOY=1` | Skip local plugin copy in `scripts/deploy-plugins-local.mjs` |

## Pull requests and AI tools

Write commit messages and PR titles in English ([Conventional Commits](https://www.conventionalcommits.org/) like `feat(scope): ...`, `fix: ...`, `docs: ...` preferred). Keep each PR focused on one change, update translated READMEs (`README.ja.md`, `README_pt_br.md`, `README.ru.md`, `README_zh_CN.md`) when changing user-facing features in [README.md](README.md), and fill out [.github/pull_request_template.md](.github/pull_request_template.md).

AI coding tools are allowed if you read every line you submit, run the checks above and verify the behavior in the running app yourself, tick the AI disclosure box in the PR template, and keep diffs small. Bug and security reports must describe reproducible behavior you observed directly.

Everyone participating in issues, pull requests, translations, and chat follows [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md).
