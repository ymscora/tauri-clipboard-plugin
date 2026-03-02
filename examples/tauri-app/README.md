# Clipboard Pro Example (Vue3 + Tauri v2)

This example app is a real Tauri project to test all `tauri-plugin-clipboard-pro` APIs.

Published JS package name: `tauri-plugin-clipboard-pro-api`.

## 1. Install

```bash
cd examples/tauri-app
npm install
```

## 2. Run

```bash
npm run tauri dev
```

## 3. API Coverage In UI

The test panel covers all APIs:

- Watch: `startWatch`, `stopWatch`, `onClipboardChange`
- Format checks: `availableFormats`, `hasText`, `hasRtf`, `hasHtml`, `hasImage`, `hasFiles`, `hasFormat`
- Read: `readText`, `readRtf`, `readHtml`, `readImage`, `readFiles`, `readBuffer`, `readClipboard`
- Write: `writeText`, `writeRtf`, `writeHtml`, `writeImage`, `writeImageBytes`, `writeFiles`, `writeVideoFiles`, `writeBuffer`
- Maintenance: `clear`, `getFilePath`

## 4. Detailed Real Test Cases

1. Watcher flow
- Click `startWatch`.
- Use external apps to copy text/image/files.
- Confirm `onClipboardChange` logs appear.
- Click `stopWatch`, confirm no new watcher logs.

2. Text/RTF/HTML roundtrip
- Use write controls for `writeText`, `writeRtf`, `writeHtml`.
- Then click `readText`, `readRtf`, `readHtml`.
- Confirm returned content matches expected value.

3. Image write/read (path)
- Fill absolute image path in `writeImage(path)`.
- Click `writeImage(path)`, then `readImage`.
- Verify width/height/size/path are valid.

4. Image write/read (bytes)
- Choose a local image file in file input.
- This triggers `writeImageBytes(file)`.
- Click `readImage` and check metadata and bytes output (`includeBytes=true`).

5. Files and video-files path list
- Fill multi-line paths and run `writeFiles`.
- Run `readFiles`, verify items and size.
- Fill video-only paths and run `writeVideoFiles`.
- For non-video extension, verify API returns error instead of app crash.

6. Custom raw buffer
- Set format like `application/x-demo-bin`.
- Fill hex bytes and run `writeBuffer`.
- Run `readBuffer` with same format.
- Verify payload bytes match.

7. Snapshot and format inspection
- Run `availableFormats`.
- Run `hasFormat` with formats (`image/png`, `text/plain`, custom format).
- Run `readClipboard` and confirm aggregated result.

8. Cleanup
- Run `getFilePath` to confirm cache path.
- Run `clear` and then run has/read APIs to confirm empty state behavior.

## 5. Notes

- For files/video tests, input should be absolute local paths.
- Clipboard support may vary slightly by OS and desktop environment.
- This app logs action latency in milliseconds to help compare read/write speed.
- This demo imports the local workspace guest API alias; production apps should install and import `tauri-plugin-clipboard-pro-api`.
