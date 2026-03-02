# API Reference

English | 中文

## Core Commands

- `start_watch` / `stop_watch`: start or stop clipboard change watcher.
- `available_formats`: list current clipboard formats.
- `has_text` / `has_rtf` / `has_html` / `has_image` / `has_files` / `has_format`.
- `read_text` / `read_rtf` / `read_html` / `read_image` / `read_files` / `read_buffer` / `read_clipboard`.
- `write_text` / `write_rtf` / `write_html` / `write_image` / `write_image_bytes` / `write_files` / `write_video_files` / `write_buffer`.
- `clear` / `get_file_path`.

## Typical Flow

1. Write file/video paths quickly:

```ts
import { writeFiles } from './guest-js/index';

await writeFiles(['/path/to/a.mp4', '/path/to/b.png']);
```

2. Write raw binary format:

```ts
import { writeBuffer } from './guest-js/index';

await writeBuffer({
  format: 'application/x-my-binary',
  data: [1, 2, 3, 4]
});
```

3. Read snapshot with low cognitive load:

```ts
import { readClipboard } from './guest-js/index';

const snapshot = await readClipboard({ preferRawPng: true });
console.log(snapshot.availableFormats, snapshot.text, snapshot.files);
```

## Error Object

Every failing command returns:

```json
{
  "code": "CLIPBOARD_BACKEND_ERROR",
  "message": "clipboard backend error: ..."
}
```
