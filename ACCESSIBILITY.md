# Accessibility

## What works today

Under **Settings → General → Accessibility**, LibreGet has switches for **Reduce motion** (cuts animations down to `0.01ms` transitions and gentle spinner fades) and **Reduce transparency** (turns off backdrop blur and uses solid backgrounds). Components and routes also follow the OS `prefers-reduced-motion: reduce` setting.

[src/app.css](src/app.css) adjusts borders and dimmed text automatically when `@media (prefers-contrast: more)` is active. LibreGet includes 14 themes (including Light, Dark, and three E-Ink variants), and `node scripts/contrast-audit.mjs` checks theme tokens against WCAG 4.5:1 contrast targets.

The interface is translated into 12 languages (`en`, `ru`, `el`, `pt`, `zh`, `zh-TW`, `ja`, `it`, `fr`, `es`, `fa`, `lo`) on [Weblate](https://hosted.weblate.org/engage/libreget/). Picking a right-to-left language (`fa`) sets `dir="rtl"` on the document root automatically.

For keyboard navigation, **Ctrl+K** / **Cmd+K** opens the Command Palette to jump between pages, settings, and tools, and **Ctrl+Shift+S** / **Cmd+Shift+S** toggles the sidebar. Status toasts and progress banners use `aria-live="polite"` or `role="alert"`.

Every pull request runs `svelte-check` (`pnpm check`), and the repo includes [scripts/a11y-audit.mjs](scripts/a11y-audit.mjs) (checking reduced motion, click-target minimums, and `focus-visible` outlines) alongside [scripts/contrast-audit.mjs](scripts/contrast-audit.mjs). If the GUI gets in the way, most download and tool actions are also exposed via `libreget-cli` and the local MCP server (37 tools on `127.0.0.1:47720`).

LibreGet targets Windows 10 and 11, macOS 10.15+ (Apple Silicon and Intel), and Linux (x86_64 and ARM64).

## Known gaps

LibreGet has not had a formal third-party WCAG audit, and we have not yet had daily VoiceOver, NVDA, or Orca users test the whole app from start to finish.

Nine Svelte files under `src/` still have `svelte-ignore a11y_` comments, mostly on modal/overlay click-to-dismiss backdrops, drag resize handles, and media player controls in `TelegramMediaPlayer.svelte` and `PlayerShell.svelte`.

The built-in video players display subtitles that came with or were downloaded alongside a video; they do not generate live captions on their own (offline transcription is available separately under **Tools → Speech and subtitles → Transcribe** with `whisper.cpp`).

## Reporting an issue

If a missing label, keyboard trap, contrast bug, or screen-reader issue stops you from using something in LibreGet, treat it as a bug and open a [Bug report](https://github.com/OpenSelena/libreget/issues/new/choose) with your LibreGet version, OS, the screen or control involved, and the assistive tech you use. If you don't have a GitHub account, you can reach us on [Telegram](https://t.me/OpenSelena).

## Notes for contributors

Keep `pnpm check` clean and do not add `svelte-ignore a11y_` comments to silence warnings on interactive controls users need to reach—use semantic elements, accessible names, and keyboard handlers instead. Respect `data-reduce-motion="true"` and `@media (prefers-reduced-motion: reduce)` for new animations, follow the target sizes and focus outlines in [DESIGN.md](DESIGN.md), and put all UI copy in `src/lib/i18n/en.json` rather than hardcoding English strings in components.
