# tauri-plugin-clipboard-pro

[![Crates.io](https://img.shields.io/crates/v/tauri-plugin-clipboard-pro)](https://crates.io/crates/tauri-plugin-clipboard-pro)
[![npm](https://img.shields.io/npm/v/tauri-plugin-clipboard-pro-api)](https://www.npmjs.com/package/tauri-plugin-clipboard-pro-api)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

High-performance clipboard plugin for **Tauri v2**.

English | [简体中文](./README.zh-CN.md)

## Features

- Cross-platform: Windows / macOS / Linux.
- Read/write: text, RTF, HTML, image, files, video file paths.
- Raw binary clipboard formats: `writeBuffer` / `readBuffer`.
- Structured error return (`code` + `message`) instead of panic.
- Performance-oriented fast paths for large media payloads.

## Install

### Rust (crates.io)

```bash
cargo add tauri-plugin-clipboard-pro
```

### JavaScript (npm)

```bash
npm install tauri-plugin-clipboard-pro-api
```

## Tauri Setup

Register plugin in `src-tauri/src/lib.rs`:

```rust
pub fn run() {
  tauri::Builder::default()
    .plugin(tauri_plugin_clipboard_pro::init())
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
```

Grant permission in `src-tauri/capabilities/default.json`:

```json
{
  "permissions": [
    "clipboard-pro:default"
  ]
}
```

## JS Usage

```ts
import { readText, writeText, readImage } from 'tauri-plugin-clipboard-pro-api';

await writeText('hello');
const text = await readText();
const image = await readImage({ preferRawPng: true, autoSave: false });
```

## API Coverage

- Watch: `startWatch`, `stopWatch`, `onClipboardChange`
- Check: `availableFormats`, `hasText`, `hasRtf`, `hasHtml`, `hasImage`, `hasFiles`, `hasFormat`
- Read: `readText`, `readRtf`, `readHtml`, `readImage`, `readFiles`, `readBuffer`, `readClipboard`
- Write: `writeText`, `writeRtf`, `writeHtml`, `writeImage`, `writeImageBytes`, `writeFiles`, `writeVideoFiles`, `writeBuffer`
- Utility: `clear`, `getFilePath`

## Version Alignment

Keep Rust and JS packages on the same Tauri major/minor line:

- `tauri` (Rust): `2.10.x`
- `@tauri-apps/api` (App dependency): `2.10.x`
- `@tauri-apps/cli` (App dependency): `2.10.x`

## Publishing

- Publishing guide: [docs/PUBLISHING.md](./docs/PUBLISHING.md)
- Crate: `tauri-plugin-clipboard-pro`
- NPM package: `tauri-plugin-clipboard-pro-api`

One-click release:

```bash
./scripts/release.sh 0.1.1
```

## Testing

```bash
cargo test
cargo test --test real_scenarios -- --ignored
```

- API docs: [docs/API.md](./docs/API.md)
- Scenarios: [docs/TEST_SCENARIOS.md](./docs/TEST_SCENARIOS.md)
- Real example app: [examples/tauri-app/README.md](./examples/tauri-app/README.md)
