# Dependency security updates (2026-11-22)

The compatibility branch keeps Vite 6 and SvelteKit 2, updates to the latest
patched releases in those lines, and removes the vulnerable versions detected
for Vitest, undici, cookie, KaTeX, picomatch and uuid. The Rust workspace updates
HTTP/2, QUIC, TLS trust roots and verification, SQLite, ZIP and the Wayland
protocol generator. ZIP readers now handle invalid entry names explicitly.

## Scanner findings still present

- [RUSTSEC-2026-0194](https://rustsec.org/advisories/RUSTSEC-2026-0194.html)
  and [RUSTSEC-2026-0195](https://rustsec.org/advisories/RUSTSEC-2026-0195.html):
  `librqbit-upnp 1.0.0` still requires `quick-xml 0.37`. Upgrading librqbit to
  9.1 removes that version but introduces a different vulnerable transitive
  dependency, [ringbuf](https://rustsec.org/advisories/RUSTSEC-2026-0293.html),
  and changes the session API. Until a verified update is available, disable
  torrent UPnP on untrusted networks.
- The npm scan still reports SvelteKit advisories
  [GHSA-pfhg-q3qv-9q6r](https://github.com/advisories/GHSA-pfhg-q3qv-9q6r),
  [GHSA-27mw-82g8-phjj](https://github.com/advisories/GHSA-27mw-82g8-phjj),
  [GHSA-3r8g-jcqq-3p5j](https://github.com/advisories/GHSA-3r8g-jcqq-3p5j),
  and [GHSA-6cf5-gc4q-fm3x](https://github.com/advisories/GHSA-6cf5-gc4q-fm3x).
  The desktop app has no SvelteKit server, remote functions or enhanced form
  actions. This reduces exposure; it does not remove the scanner findings.
  The major upgrade requires migrating Vite and its Svelte plugin together.

Run `pnpm audit` at the repository root and `cargo audit` from `src-tauri`.
Do not suppress these findings to make an audit pass.
