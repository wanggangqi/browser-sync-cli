# Browser Sync CLI

A browser extension + local application combo for real-time bookmark synchronization.

## Features

- **Real-time sync**: Automatically syncs browser bookmarks to local JSON file
- **Search**: Quick search through all bookmarks
- **Import/Export**: Backup and restore bookmarks
- **Cross-browser**: Supports Chrome and Edge

## Architecture

```
┌─────────────────┐     Native Messaging     ┌─────────────────┐
│  Browser        │ ─────────────────────── │  Native Host    │
│  Extension      │                         │  (Rust)         │
└─────────────────┘                         └────────┬────────┘
                                                     │
                                                     │ writes JSON
                                                     ▼
┌─────────────────┐                         ┌─────────────────┐
│  Tauri App      │ ◄─── file watching ──── │  bookmarks.json │
│  (Vue 3)        │                         └─────────────────┘
└─────────────────┘
```

## Installation

### 1. Install Native Host

```powershell
cd packages/native-host
cargo build --release
.\install.ps1
```

### 2. Install Browser Extension

1. Open Chrome/Edge
2. Go to `chrome://extensions` (or `edge://extensions`)
3. Enable "Developer mode"
4. Click "Load unpacked"
5. Select the `packages/extension` folder
6. Note the extension ID
7. Edit `%LOCALAPPDATA%\browser-sync-cli\com.browser_sync.cli.json`
8. Add your extension ID to `allowed_origins`

### 3. Run Tauri App

```bash
cd packages/app
pnpm install
pnpm tauri dev
```

## Data File Location

- **Windows**: `%LOCALAPPDATA%\browser-sync-cli\bookmarks.json`

## Development

```bash
# Install dependencies
pnpm install

# Build extension
pnpm build:extension

# Build native host
pnpm build:native-host

# Run Tauri app in dev mode
pnpm --filter app tauri dev
```

## License

MIT