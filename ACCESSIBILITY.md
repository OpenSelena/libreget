# Accessibility

LibreGet aims to be usable with a keyboard, assistive technology, reduced motion and transparency preferences, high-contrast themes, and multiple languages. This document describes what LibreGet implements today, where known limitations remain, and how to report accessibility barriers.

## What LibreGet supports today

- **Reduce motion**: **Settings → General → Accessibility → Reduce motion** switches position and scale animations to minimal transitions (`0.01ms`) and gentler spinner fades. Many components and routes also honor the operating system's `prefers-reduced-motion: reduce` setting automatically.
- **Reduce transparency**: **Settings → General → Accessibility → Reduce transparency** replaces translucent surfaces and backdrop blur with solid panel backgrounds.
- **High contrast and 14 themes**: [src/app.css](src/app.css) includes `@media (prefers-contrast: more)` adjustments for borders and dimmed text, and LibreGet ships 14 themes (including Light, Dark, and three E-Ink variants). Theme color tokens are checked against WCAG 4.5:1 contrast targets with `node scripts/contrast-audit.mjs`.
- **12 languages and RTL layout**: Interface copy is localized in 12 languages (`en`, `ru`, `el`, `pt`, `zh`, `zh-TW`, `ja`, `it`, `fr`, `es`, `fa`, `lo`) on [Weblate](https://hosted.weblate.org/engage/libreget/). Selecting a right-to-left locale (`fa`) automatically sets `dir="rtl"` on the root document.
- **Keyboard navigation and announcements**: **Ctrl+K** / **Cmd+K** opens the Command Palette for quick keyboard navigation across pages, settings, and tools, and **Ctrl+Shift+S** / **Cmd+Shift+S** toggles the sidebar. Status notifications and progress banners use `aria-live="polite"` or `role="alert"` regions.
- **Automated checks**: Every pull request runs `svelte-check` (`pnpm check`), and the repository includes [scripts/a11y-audit.mjs](scripts/a11y-audit.mjs) (checking reduced-motion adherence, interactive click-target sizes, and `focus-visible` behavior) and [scripts/contrast-audit.mjs](scripts/contrast-audit.mjs).
- **Non-GUI alternatives**: Core downloading and tool workflows are also exposed through `libreget-cli` and the local MCP server (37 tools on `127.0.0.1:47720`) for users who prefer terminal or agent-driven workflows.

## Supported environments

LibreGet targets desktop environments on:

- Windows 10 and 11
- macOS 10.15+ (Apple Silicon and Intel)
- Linux (x86_64 and ARM64)

## Known limitations

- **No external accessibility audit**: LibreGet has not undergone a formal third-party WCAG conformance audit and does not claim formal WCAG certification.
- **Screen-reader coverage**: The full application has not yet been audited end-to-end by daily VoiceOver, NVDA, or Orca users.
- **Remaining lint suppressions**: Nine Svelte files under `src/` currently contain `svelte-ignore a11y_` comments (primarily modal/overlay click-to-dismiss backdrops, drag resize handles, and embedded media player controls in `TelegramMediaPlayer.svelte` and `PlayerShell.svelte`).
- **Media captions**: Built-in players render subtitles included with or downloaded alongside media files. They do not auto-generate real-time captions for uncaptioned media (offline subtitle generation is available separately via **Tools → Speech and subtitles → Transcribe** using `whisper.cpp`).

## Reporting an accessibility barrier

Accessibility barriers are treated as bugs:

- Open a [Bug report](https://github.com/OpenSelena/libreget/issues/new/choose) and include your LibreGet version, operating system, the screen or control affected, steps to reproduce, and the assistive technology or input method you use.
- If you cannot use GitHub, reach out via [Telegram](https://t.me/OpenSelena).

## Expectations for contributors

- Keep `pnpm check` (`svelte-check`) clean without introducing new accessibility warnings.
- Do not add `svelte-ignore a11y_` comments to silence warnings on interactive controls users need to reach; provide proper semantic elements, accessible labels, and keyboard handlers instead.
- Respect `data-reduce-motion="true"` and `@media (prefers-reduced-motion: reduce)` for new animations, and keep interactive target sizes and focus outlines consistent with [DESIGN.md](DESIGN.md).
- Always place new user-facing text in `src/lib/i18n/en.json` rather than hardcoding strings in components so labels can be translated and read by screen readers in every supported language.
