# Accessibility

LibreGet is a desktop app for Windows, macOS, and Linux. This page explains what accessibility features work today, where the app falls short, and how to report barriers.

## Current support

- Appearance settings (`Settings → Appearance`) include toggles for Reduce motion and Reduce transparency, and many animations follow your system's reduced-motion setting.
- 14 built-in themes (Light, Dark, E-Ink, and dark/light community palettes) are checked for text contrast, and borders sharpen when your OS requests higher contrast.
- Typography settings (`Settings → Typography`) let you switch display, body, and monospace fonts to a system font and adjust line height (`1.30`–`1.90`) and spacing (`0.85×`–`1.25×`).
- Keyboard navigation includes `Ctrl+K` (`Cmd+K` on macOS) for the Command Palette, `Ctrl+1` / `Ctrl+2` for Home and Downloads, `Ctrl+,` for Settings, `Ctrl+Shift+S` to toggle the sidebar, `?` to view shortcuts, and configurable global download hotkeys (which require Accessibility permission on macOS).
- Error and status notifications announce through live status regions, core modal dialogs restore focus when closed, and sidebar navigation uses labeled landmarks.
- For non-GUI workflows, `libreget-cli` covers downloading, batching, search, and cookies from a terminal, and the local MCP server exposes 37 tools.

## Known gaps

- LibreGet has not had a formal WCAG audit or a full end-to-end test pass with NVDA, VoiceOver, or Orca.
- There is no "Skip to main content" link, and no global UI zoom or base font-size slider yet (only line height and spacing scale).
- Some custom overlays, drag-resize bars, and Telegram panels lack full focus trapping or keyboard controls, and the course player seek bar is pointer-only.
- Parts of the course video player and shortcut overlay still use hardcoded Portuguese `aria-label` text instead of localized strings, and a few icon buttons use raw symbols (`↑`, `↓`, `×`) as labels.
- Only `ru`, `zh`, `lo`, and `pt` are nearly fully translated on [Weblate](https://hosted.weblate.org/engage/libreget/); other locales fall back to English for missing keys, and Persian (`fa`) RTL layout mirroring is only partial.
- Built-in video players show subtitles that come with or alongside a file rather than auto-generating captions (offline Whisper transcription is separate under `Tools → Speech and subtitles`).

## Reporting barriers

If a missing label, keyboard trap, contrast issue, or screen-reader bug stops you from using LibreGet, open a [Bug report](https://github.com/OpenSelena/libreget/issues/new/choose) with your app version, OS, the screen or control involved, and the assistive tool you use. Without a GitHub account, reach us on [Telegram](https://t.me/OpenSelena).

## Notes for contributors

- Do not add `svelte-ignore a11y_` comments to silence warnings on controls users need to reach—use proper semantic elements, accessible labels, and keyboard handlers.
- Localize all UI and `aria-label` strings in `src/lib/i18n/en.json` (`pnpm generate:i18n-keys`) instead of hardcoding English or Portuguese strings.
- Respect reduced-motion preferences in new animations, keep interactive sizes and focus outlines aligned with [DESIGN.md](DESIGN.md), and verify theme contrast changes with `node scripts/contrast-audit.mjs`.
