# tauri-plugin-clipboard-pro

[![Crates.io](https://img.shields.io/crates/v/tauri-plugin-clipboard-pro)](https://crates.io/crates/tauri-plugin-clipboard-pro)
[![npm](https://img.shields.io/npm/v/tauri-plugin-clipboard-pro-api)](https://www.npmjs.com/package/tauri-plugin-clipboard-pro-api)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

面向 **Tauri v2** 的高性能剪贴板插件。

[English](./README.md) | 简体中文

## 功能特性

- 跨平台支持：Windows / macOS / Linux。
- 读写能力：文本、RTF、HTML、图片、文件、视频路径。
- 支持原始二进制格式：`writeBuffer` / `readBuffer`。
- 异常统一结构化返回（`code` + `message`），避免 panic。
- 针对大媒体数据提供性能优先的 fast path。

## 安装

### Rust（crates.io）

```bash
cargo add tauri-plugin-clipboard-pro
```

### JavaScript（npm）

```bash
npm install tauri-plugin-clipboard-pro-api
```

## Tauri 接入

在 `src-tauri/src/lib.rs` 注册插件：

```rust
pub fn run() {
  tauri::Builder::default()
    .plugin(tauri_plugin_clipboard_pro::init())
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
```

在 `src-tauri/capabilities/default.json` 添加权限：

```json
{
  "permissions": [
    "clipboard-pro:default"
  ]
}
```

## JS 使用示例

```ts
import { readText, writeText, readImage } from 'tauri-plugin-clipboard-pro-api';

await writeText('hello');
const text = await readText();
const image = await readImage({ preferRawPng: true, autoSave: false });
```

## API 覆盖

- 监听：`startWatch`、`stopWatch`、`onClipboardChange`
- 检查：`availableFormats`、`hasText`、`hasRtf`、`hasHtml`、`hasImage`、`hasFiles`、`hasFormat`
- 读取：`readText`、`readRtf`、`readHtml`、`readImage`、`readFiles`、`readBuffer`、`readClipboard`
- 写入：`writeText`、`writeRtf`、`writeHtml`、`writeImage`、`writeImageBytes`、`writeFiles`、`writeVideoFiles`、`writeBuffer`
- 工具：`clear`、`getFilePath`

## 版本对齐要求

Rust 和 JS 侧需保持同一 Tauri major/minor：

- `tauri`（Rust）：`2.10.x`
- `@tauri-apps/api`（应用依赖）：`2.10.x`
- `@tauri-apps/cli`（应用依赖）：`2.10.x`

## 发布说明

- 发布文档：[docs/PUBLISHING.md](./docs/PUBLISHING.md)
- Crate 名称：`tauri-plugin-clipboard-pro`
- npm 包名：`tauri-plugin-clipboard-pro-api`

一键发布命令：

```bash
./scripts/release.sh 0.1.1
```

## 测试

```bash
cargo test
cargo test --test real_scenarios -- --ignored
```

- API 文档：[docs/API.md](./docs/API.md)
- 场景文档：[docs/TEST_SCENARIOS.md](./docs/TEST_SCENARIOS.md)
- 真实示例应用：[examples/tauri-app/README.zh-CN.md](./examples/tauri-app/README.zh-CN.md)
