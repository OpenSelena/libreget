# Accessibility

LibreGet is a desktop app for Windows 10/11, macOS 10.15+ (Apple Silicon and Intel), and Linux (x86_64 and ARM64). This page lists what accessibility support exists today, what is still missing, and how to report barriers.

## What works today

- **Settings → Appearance → Accessibility** has toggles for Reduce motion (turns position and scale animations into simple fades) and Reduce transparency (uses solid backgrounds instead of translucent panels). Many routes and components also follow the OS `prefers-reduced-motion: reduce` preference.
- 14 built-in themes (Light, Dark, Catppuccin, Dracula, One Dark Pro, three E-Ink themes, and three Nyxvamp themes) meet WCAG 4.5:1 text contrast targets in `node scripts/contrast-audit.mjs`, and [src/app.css](src/app.css) increases border and muted-text contrast when OS `prefers-contrast: more` is active.
- 12 languages are wired up on [Weblate](https://hosted.weblate.org/engage/libreget/) (`en`, `ru`, `el`, `pt`, `zh`, `zh-TW`, `ja`, `it`, `fr`, `es`, `fa`, `lo`), with automatic `dir="rtl"` root layout for Persian (`fa`).
- `Ctrl+K` / `Cmd+K` opens the Command Palette to jump to any page, setting, or tool, and `Ctrl+Shift+S` / `Cmd+Shift+S` toggles the sidebar (displayed as `⌃⌘S` in the macOS sidebar tooltip).
- Status toasts and update banners expose live status announcements (`aria-live="polite"` and `role="alert"`).
- `libreget-cli` and the 37-tool local MCP server (`127.0.0.1:47720`) cover core downloads and tools from a terminal or AI client when the GUI is inconvenient.

## Known limitations

- No formal third-party WCAG audit has been done, and LibreGet has not yet been tested end-to-end by daily VoiceOver, NVDA, or Orca users.
- Some pointer-oriented areas still have keyboard or label gaps (modal/overlay click backdrops, drag resize bars, and custom controls in the Telegram and media players). `svelte-check` (`pnpm check`) currently reports 36 non-fatal compiler warnings across 17 files, including `a11y_media_has_caption` and `a11y_autofocus`.
- Several Weblate locales still fall back to English for newer settings and tools until community translations catch up.
- Built-in video players show existing or downloaded subtitles rather than generating live captions (offline Whisper transcription is separate under **Tools → Speech and subtitles → Transcribe**).

## Reporting a barrier

If a missing label, keyboard trap, contrast issue, or screen-reader bug blocks you from using a feature, open a [Bug report](https://github.com/OpenSelena/libreget/issues/new/choose) with your LibreGet version, OS, the screen or control affected, steps to reproduce, and the assistive technology you use. Without a GitHub account, reach out on [Telegram](https://t.me/OpenSelena).

## For contributors

- Keep `pnpm check` at zero errors, do not introduce new accessibility warnings, and avoid adding `svelte-ignore a11y_` comments to skip keyboard or label fixes on controls users need to reach.
- Support `data-reduce-motion="true"` and `@media (prefers-reduced-motion: reduce)` for new animations, follow the control sizing and focus rings in [DESIGN.md](DESIGN.md), and check theme token edits with `node scripts/contrast-audit.mjs`.
- Put new interface strings in `src/lib/i18n/en.json` and run `pnpm generate:i18n-keys` instead of hardcoding English text in Svelte components.
