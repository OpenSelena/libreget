# Security Policy

## Supported Versions

Only the latest minor release receives active security updates and patches.

| Version | Supported |
| ------- | --------- |
| 1.0.x  | ✅        |
| < 1.0  | ❌        |

## Reporting a Vulnerability

**Please do not open a public issue for security vulnerabilities.**

To report a vulnerability responsibly:

1. **GitHub Private Vulnerability Reporting (Preferred)**:
   Navigate to the [LibreGet Security Advisories](https://github.com/OpenSelena/libreget/security/advisories/new) page and click **"Report a vulnerability"**. This allows secure, private collaboration and testing of a fix before disclosure.

2. **Email**:
   If you cannot use GitHub advisories, email **igect@vk.com** with:
   - A detailed description of the vulnerability
   - Step-by-step reproduction instructions or proof-of-concept (PoC)
   - Affected LibreGet version(s) and operating system
   - Your assessment of severity and impact

### Response Timeline

- **Initial Response**: Within 24-48 hours acknowledging receipt.
- **Triage & Confirmation**: Within 3-5 days.
- **Fix & Disclosure**: If confirmed, a fix will be published in the next patch release, and you will be credited in the release notes (unless you prefer to remain anonymous).

## Scope

### In scope

- The LibreGet desktop application (`src-tauri/` and `src/`), `libreget-cli`, and the local MCP server
- The companion browser extension (`browser-extension/`) and local bridge (`127.0.0.1:47720–47729`)
- Embedded core plugins (`courses`, `telegram`, `convert`)
- The updater and signature verification, as well as how LibreGet downloads and verifies managed external tools (`yt-dlp`, `FFmpeg`, `gallery-dl`, `aria2`)

### Out of scope

- Vulnerabilities inside third-party tools themselves (`yt-dlp`, `FFmpeg`, `gallery-dl`, `aria2`, etc.) — report those upstream (please also notify us if LibreGet exposes or worsens the risk)
- Third-party websites that LibreGet downloads from
- Attack scenarios that assume an attacker already has control of your local user account on the machine
