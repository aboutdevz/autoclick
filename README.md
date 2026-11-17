# AutoClick

A modern, lightweight Windows desktop auto-clicker built with **Rust** and **Tauri v2**.

## Features

- **Precision Interval**: Configure `Hours : Minutes : Seconds : Milliseconds` (down to `1ms`) or use quick presets (`10ms`, `50ms`, `100ms`, `500ms`, `1s`).
- **Random Jitter**: Optional `± ms` randomized delay per click for natural timing.
- **Click Options**: Left, Right, or Middle mouse button; Single, Double, or Hold click type; Infinite or Fixed Count repeat.
- **Cursor Position**: Follow current cursor or click at a Fixed `(X, Y)` coordinate with an interactive **Pick Location** mode.
- **Global Hotkeys**: Customizable shortcut (default `F6`) with **Toggle** (press to start/stop) or **Hold** (click while held) modes.
- **System Tray & Startup**: Minimize to system tray on close, start minimized to tray, tray context menu, and launch on Windows startup.
- **Modern UI/UX**: Custom frameless titlebar, Dark / Light / System themes, and **English / Bahasa Indonesia** (`EN` / `ID`) language switcher.
- **Installer & Portable**: Available as a Windows NSIS Installer (`*-setup.exe`) and a Portable build (`*-portable.zip`).

## Installation vs Portable Mode

- **Installer (`AutoClick_*_x64-setup.exe`)**: Installs AutoClick for the current user and stores settings in `%APPDATA%\com.aboutdevz.autoclick\config.json`.
- **Portable (`AutoClick_*_x64-portable.zip`)**: Extract anywhere and run `AutoClick.exe`. Because `portable.flag` is included next to the executable, all settings are saved locally in `config.json` in the same folder. You can also pass `--portable` on the command line.

## Releases

GitHub Releases are automatically built and published via [`.github/workflows/release.yml`](.github/workflows/release.yml) whenever a version tag (`v*`, e.g., `v1.0.0`) is pushed:

```bash
git tag v1.0.0
git push origin v1.0.0
```

## Development

### Prerequisites
- [Node.js](https://nodejs.org/) (v20+)
- [Rust](https://www.rust-lang.org/tools/install) (stable)
- Windows 10/11 (with WebView2 Runtime, pre-installed on Windows 10/11)

### Commands

```bash
# Install frontend dependencies
npm install

# Run core Rust unit tests
cd autoclick-core && cargo test

# Run desktop app in development mode (on Windows)
npm run tauri dev

# Build production NSIS installer & executable (on Windows)
npm run tauri build
```

## License

MIT — see [LICENSE](LICENSE).
