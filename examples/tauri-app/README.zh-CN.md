# Clipboard Pro 示例应用（Vue3 + Tauri v2）

这是一个真实的 Tauri 示例工程，用于覆盖测试 `tauri-plugin-clipboard-pro` 的全部 API。

发布到 npm 的 JS 包名为：`tauri-plugin-clipboard-pro-api`。

## 1. 安装

```bash
cd examples/tauri-app
npm install
```

## 2. 运行

```bash
npm run tauri dev
```

## 3. UI 覆盖 API

- 监听：`startWatch`、`stopWatch`、`onClipboardChange`
- 格式探测：`availableFormats`、`hasText`、`hasRtf`、`hasHtml`、`hasImage`、`hasFiles`、`hasFormat`
- 读取：`readText`、`readRtf`、`readHtml`、`readImage`、`readFiles`、`readBuffer`、`readClipboard`
- 写入：`writeText`、`writeRtf`、`writeHtml`、`writeImage`、`writeImageBytes`、`writeFiles`、`writeVideoFiles`、`writeBuffer`
- 维护：`clear`、`getFilePath`

## 4. 详尽真实测试场景

1. 监听场景
- 点击 `startWatch`。
- 在外部应用复制文本/图片/文件。
- 确认日志中出现 `onClipboardChange`。
- 点击 `stopWatch` 后确认不再继续触发。

2. 文本/RTF/HTML 往返
- 分别执行 `writeText`、`writeRtf`、`writeHtml`。
- 再执行 `readText`、`readRtf`、`readHtml`。
- 比对返回内容是否一致。

3. 图片路径写入读取
- 在 `writeImage(path)` 输入绝对路径。
- 点击写入后执行 `readImage`。
- 验证宽高、大小、缓存路径是否正确。

4. 图片字节写入读取
- 通过文件输入框选择本地图片（触发 `writeImageBytes`）。
- 执行 `readImage`，可开启 `includeBytes`。
- 验证 bytes 与元数据。

5. 文件与视频路径
- 多行输入文件路径执行 `writeFiles`。
- 执行 `readFiles`，检查路径和 size。
- 多行输入视频路径执行 `writeVideoFiles`。
- 输入非视频路径应返回错误，不应崩溃。

6. 原始二进制格式
- 设置格式为 `application/x-demo-bin`。
- 十六进制输入字节并执行 `writeBuffer`。
- 用相同格式执行 `readBuffer`。
- 验证读写字节一致。

7. 快照与格式总览
- 执行 `availableFormats`。
- 使用 `hasFormat` 测试常见/自定义格式。
- 执行 `readClipboard` 验证聚合结果。

8. 收尾清理
- 执行 `getFilePath` 查看插件缓存目录。
- 执行 `clear` 后再次 `has/read` 验证清空行为。

## 5. 注意事项

- 文件与视频测试请使用绝对路径。
- 不同平台桌面环境对剪贴板格式支持存在差异。
- 面板会记录 API 调用耗时（毫秒）用于性能对比。
- 此示例通过工作区本地 alias 引入 guest API；生产环境请安装并引用 `tauri-plugin-clipboard-pro-api`。
