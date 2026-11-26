# Contributing to LibreGet

- Questions and help: [Discussions](https://github.com/OpenSelena/libreget/discussions) or [Telegram](https://t.me/OpenSelena)
- Bugs and feature requests: [Issues](https://github.com/OpenSelena/libreget/issues/new/choose)
- Security vulnerabilities: [SECURITY.md](SECURITY.md) (do not open a public issue)

## Before writing code

Read [GLOSSARY.md](GLOSSARY.md), [DESIGN.md](DESIGN.md), and [docs/adr/](docs/adr) for canonical naming, UI tokens, and existing architecture decisions. UI changes must follow [ACCESSIBILITY.md](ACCESSIBILITY.md). Comment on an open issue before starting work so two people do not solve the same thing, and open a [discussion](https://github.com/OpenSelena/libreget/discussions) before implementing large features or new plugins.

LibreGet does not accept changes that bypass DRM, break paywalls, or expose user credentials. Add new UI strings only in `src/lib/i18n/en.json`; translations for the other 11 locales are managed on [Weblate](https://hosted.weblate.org/engage/libreget/) (configured in `.weblate`), so do not submit machine-translated locale files in feature PRs.

## Setup

- <a href="https://rustup.rs/"><img src="https://cdn.simpleicons.org/rust/CE422B" width="20" height="20" align="center" alt="Rust" title="Rust" /></a> 1.97.0 — pinned in [rust-toolchain.toml](rust-toolchain.toml) (`rustfmt` + `clippy`) so host and plugin ABIs match
- <a href="https://nodejs.org/"><img src="https://api.iconify.design/logos/nodejs-icon.svg" width="20" height="20" align="center" alt="Node.js" title="Node.js" /></a> 18+ (LTS) — CI runs `lts/*` in [.github/workflows/ci.yml](.github/workflows/ci.yml)
- <a href="https://pnpm.io/"><img src="https://api.iconify.design/tabler/brand-pnpm.svg?color=%23F69220" width="20" height="20" align="center" alt="pnpm" title="pnpm" /></a> 10.29.3 — set via `packageManager` in [package.json](package.json) (`corepack enable`)
- <a href="https://www.npmjs.com/"><img src="https://api.iconify.design/tabler/brand-npm.svg?color=%23CB3837" width="20" height="20" align="center" alt="npm" title="npm" /></a> bundled with Node — fallback to install pnpm (`npm i -g pnpm@10.29.3`); use `pnpm install`, not `npm install`, because `pnpm-lock.yaml` is authoritative

Platform system dependencies:

- Linux: install WebKitGTK, GTK tray, SSL, audio, PipeWire, and Clang/libclang headers (same packages CI installs in [.github/workflows/ci.yml](.github/workflows/ci.yml) and [.github/workflows/release.yml](.github/workflows/release.yml); include `xvfb` if running headless UI lint or smoke tests):

```bash
sudo apt-get update && sudo apt-get install -y \
  libwebkit2gtk-4.1-dev build-essential curl wget file libxdo-dev libssl-dev \
  libayatana-appindicator3-dev librsvg2-dev patchelf libasound2-dev \
  libpipewire-0.3-dev clang libclang-dev
```

- macOS: install Xcode Command Line Tools (`xcode-select --install`; `src-tauri/build.rs` calls `xcode-select -p` for Swift runtime paths).
- Windows: WebView2 and MSVC Build Tools (`x86_64-pc-windows-msvc`).

Clone, install dependencies, and start the dev app:

```bash
git clone https://github.com/OpenSelena/libreget.git
cd libreget
pnpm install
pnpm tauri dev
```

- `pnpm install` runs `prepare`, pointing `git config core.hooksPath` to [.githooks/](.githooks) (`pre-commit` formats staged `.rs` files; `pre-push` checks `cargo fmt`). On Linux/macOS, make both scripts executable (`chmod +x .githooks/pre-commit .githooks/pre-push`) or run `pnpm fmt:rs` before committing.
- `pnpm tauri dev` launches Vite on `http://localhost:1420` (`src-tauri/tauri.conf.json`) alongside the Tauri backend. Set `TAURI_DEV_HOST` for mobile or LAN testing (Vite HMR runs on port `1421` in `vite.config.js`).
- `pnpm dev` runs only the SvelteKit/Vite frontend when working on UI or Vitest modules without compiling Rust. Vite automatically runs `scripts/generate-i18n-keys.js` on startup (`pnpm build` runs it in strict mode and fails on locale mismatches unless `LIBREGET_I18N_STRICT=0`).
- `pnpm tauri build --config '{"bundle":{"createUpdaterArtifacts":false}}'` creates a local release build without the maintainer's `TAURI_SIGNING_PRIVATE_KEY` (`createUpdaterArtifacts` defaults to `true` in `src-tauri/tauri.conf.json`). On Linux, `pnpm tauri:appimage` wraps AppImage bundling through `scripts/linux/appimage.sh`.
- `pnpm plugins:deploy` is only for external dynamic plugin sibling checkouts (`../libreget-plugin-courses`, `../libreget-plugin-telegram`, `../libreget-plugin-convert`); built-in plugins in `src-tauri/plugins/` already compile into the app binary.
- `pnpm clean:cargo` / `pnpm clean:cargo:all` runs `cargo sweep` inside `src-tauri/` to clear stale build artifacts.

## Codebase layout

- `src/routes/`, `src/components/`, `src/lib/` — Svelte 5 / SvelteKit static SPA (`@sveltejs/adapter-static`, fallback `index.html`, aliases `$lib` and `$components`). Design tokens and 14 app themes live in `src/app.css` and `src/lib/style/`; locale files (`src/lib/i18n/*.json`) and generated `src/lib/i18n/keys.ts` live in `src/lib/i18n/`.
- `src-tauri/src/` — Tauri v2 desktop host: `commands/`, `cookies/`, `core/` (download queue, `queue_wal.rs`, portable mode `portable.rs`, rules, host limiter), `local_bridge.rs` (authenticated extension bridge on `127.0.0.1:47720–47729` plus temporary `/media/{t}` HTTP range streams for WebKitGTK), `mcp.rs` (Streamable HTTP JSON-RPC MCP server on `POST /mcp`), `hotkey.rs`, `tray.rs`, `plugin_host.rs`, and `plugin_loader.rs`. Register new Tauri commands in `.invoke_handler(tauri::generate_handler![...])` in [src-tauri/src/lib.rs](src-tauri/src/lib.rs) and declare needed permissions in [src-tauri/capabilities/default.json](src-tauri/capabilities/default.json).
- `src-tauri/libreget-core/` — Shared Rust engine (`libreget-core`): extractors in `src/platforms/`, tool backends in `src/core/tools/`, downloader/yt-dlp/FFmpeg wrappers in `src/core/`, and `desktop` / `mobile` Cargo features.
- `src-tauri/libreget-cli/` — Standalone `libreget-cli` binary (`download`, `info`, `batch`, `search`, `archive`, and `import-cookies` subcommands, with global `--json` and `--proxy` flags).
- `src-tauri/libreget-plugin-sdk/` & `src-tauri/plugins/` — Plugin SDK trait/ABI (`ABI_VERSION = 3` + `libreget_plugin_build_info` rustc fingerprint check), statically linked plugins (`convert`, `telegram`, `courses`), and local `[patch.crates-io]` `grammers-session`. Built-ins register via `PluginManager::register_builtin` in [src-tauri/src/lib.rs](src-tauri/src/lib.rs).
- `browser-extension/` — Manifest V3 Chrome (`chrome/`) and Firefox (`firefox/`) extensions bundled by `src-tauri/tauri.conf.json`. Edit shared files (`src/`, `pages/`, `popup/`, `_locales/`, `icons/`) in `browser-extension/chrome/`, sync to `firefox/` with `pnpm extension:sync` (`manifest.json` stays per-browser), and package Chrome Web Store zips with `node browser-extension/chrome/scripts/package.mjs`.
- `flatpak/` — Flatpak manifest (`com.openselena.libreget.yml`), desktop entry, offline dependency manifests, and AppStream metadata (`com.openselena.libreget.metainfo.xml`).
- `scripts/` — UI lint (`lint-ui.mjs`), accessibility and contrast audits (`a11y-audit.mjs`, `contrast-audit.mjs`), Clippy baseline (`clippy-baseline.mjs`), i18n keygen and locale sync (`generate-i18n-keys.js`, `sync-locales.mjs`), smoke runner (`smoke-test.mjs`), UI screenshot capture (`shots.mjs`), WAL crash fixture (`wal-kill9.py`), Linux AppImage helpers (`linux/`), and release version bumps (`bump-version.js`).

## Branch workflow

Target **`dev`** for all pull requests. Pushes and PRs against `dev` run GitHub Actions CI ([.github/workflows/ci.yml](.github/workflows/ci.yml)) across `ubuntu-latest`, `debian:bookworm`, `windows-latest`, and `macos-latest` (excluding `weblate` translation PRs on Rust and Flatpak jobs). Maintainers merge `dev` into `main` only after CI passes.

## Local checks

Run the checks covering the areas you changed before pushing:

### Frontend, i18n, and browser extension

```bash
pnpm check                 # svelte-kit sync + svelte-check
pnpm test                  # Vitest suite (src/**/*.test.ts in Node)
pnpm check:i18n            # strict check that all 12 locales in src/lib/i18n/ match en.json
pnpm extension:sync        # sync shared files from browser-extension/chrome/ to firefox/
node --test browser-extension/chrome/tests/*.test.mjs
```

- When adding or renaming translation keys in `src/lib/i18n/en.json`, run `pnpm generate:i18n-keys` and commit `src/lib/i18n/keys.ts` (CI fails `git diff --exit-code -- src/lib/i18n/keys.ts` if stale). `node scripts/sync-locales.mjs --check` verifies locale key parity without writing.
- Whenever modifying Svelte routes (`src/routes/`), components (`src/components/`), or CSS (`src/app.css`, `src/lib/style/`), run targeted UI lint ([.agents/skills/ui-lint/SKILL.md](.agents/skills/ui-lint/SKILL.md)):

```bash
pnpm lint:ui /settings               # single route (dark + light)
pnpm lint:ui --shell                 # all core routes against scripts/ui-lint-baseline.json
pnpm lint:ui /settings --all-themes  # all 14 themes in src/app.css
```

- Optional Playwright and theme checks: `node scripts/a11y-audit.mjs` checks `prefers-reduced-motion`, click target sizes, and keyboard focus visibility against a local Vite server; `node scripts/contrast-audit.mjs` checks WCAG contrast across the 14 themes.

### Rust

```bash
pnpm check:fmt                       # or pnpm fmt:rs to format src-tauri/
pnpm check:clippy-baseline           # checks crate|lint counts against src-tauri/clippy-baseline.<platform>.json
```

Inside `src-tauri/`:

```bash
cargo clippy --workspace --all-targets
cargo test --workspace
```

- On Windows, standalone test binaries linking Tauri/WebView without an application manifest fail with `0xc0000139`, so run `cargo test --workspace --exclude libreget-plugin-courses` (matches CI).
- Fix any new Clippy warning you introduce; do not run `pnpm update:clippy-baseline` to silence new warnings. On Linux distributions where `pkg-config` files sit in `/usr/lib64/pkgconfig`, `scripts/clippy-baseline.mjs` defaults `PKG_CONFIG_PATH` there if unset.

### Smoke test and Flatpak metainfo

```bash
node scripts/smoke-test.mjs src-tauri/target/debug/libreget
node scripts/smoke-test.mjs src-tauri/target/debug/libreget --portable
appstreamcli validate flatpak/com.openselena.libreget.metainfo.xml
```

On headless Linux, prefix smoke commands with `xvfb-run -a`. Pass `.exe` (`libreget.exe`) on Windows.

## Environment variables

Runtime overrides and portable testing:

- `LIBREGET_DATA_DIR` — overrides the profile/data directory (`libreget-core/src/core/paths.rs`, `src-tauri/src/core/portable.rs`, `scripts/deploy-plugins-local.mjs`)
- `LIBREGET_PORTABLE=1` — enables portable mode alongside `LIBREGET_DATA_DIR` (set automatically when `portable.txt` or `.portable` sits beside the executable)
- `LIBREGET_RELAY` — sets the P2P transfer relay server address (`libreget-core/src/platforms/p2p.rs`)
- `LIBREGET_POT_PROVIDER_URL`, `LIBREGET_YTDLP_CHANNEL` (`nightly` or `stable`), `LIBREGET_PYTHON` — overrides yt-dlp PO-token provider URL, release channel, or Python binary (`libreget-core/src/core/ytdlp.rs`)

Development scripts and ignored live tests:

- `TAURI_DEV_HOST` — exposes the Vite dev/HMR server (`port 1421`) for remote or mobile testing (`vite.config.js`)
- `LIBREGET_I18N_STRICT=0` / `GENERATE_I18N_KEYS_STRICT=1` — toggles strict locale validation in Vite (`vite.config.js`) and `scripts/generate-i18n-keys.js`
- `LIBREGET_SMOKE_EXIT_MS` — auto-exit timeout in milliseconds for `scripts/smoke-test.mjs` (`src-tauri/src/lib.rs`)
- `LIBREGET_SKIP_PLUGIN_DEPLOY=1` — skips sibling plugin build and copy in `scripts/deploy-plugins-local.mjs`
- `SHOTS_OUT` — overrides the `.shots` output directory for `scripts/shots.mjs`
- `#[ignore]` live Rust tests (`cargo test ... -- --ignored` in `libreget-core`): `LIBREGET_YTDLP`, `LIBREGET_E2E_URL`, `LIBREGET_IG_COOKIES`, `LIBREGET_IG_TARGET`, `LIBREGET_SUBSTACK_COOKIES`, `LIBREGET_MEDIUM_COOKIES`, `LIBREGET_IMG`, `LIBREGET_PDF`

## Pull requests and AI tools

Write commit messages and PR titles in English ([Conventional Commits](https://www.conventionalcommits.org/) such as `feat(scope): ...`, `fix: ...`, `docs: ...` preferred). Keep each PR focused on one change, update the translated READMEs (`README.ja.md`, `README_pt_br.md`, `README.ru.md`, `README_zh_CN.md`) when changing user-facing features in [README.md](README.md), and fill out [.github/pull_request_template.md](.github/pull_request_template.md).

AI coding tools are allowed if you read every line you submit, run the checks above and verify behavior in the running app yourself, check the AI disclosure box in [.github/pull_request_template.md](.github/pull_request_template.md), and keep diffs focused. Bug and security reports must describe reproducible behavior you observed directly.

Participation in issues, pull requests, translations, and community channels follows [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md).
