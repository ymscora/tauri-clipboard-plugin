# Clipboard API Test Scenarios

English | 中文

## Unit Test Coverage

- Path normalization and percent decode.
- File type and mime detection (image/video/common files).
- Write/read text workflow.
- Raw buffer write validation.
- Raw PNG image read path.
- Video-file-only validation.
- Write image from path.
- File URI normalization.

## Real Clipboard Scenarios (`tests/real_scenarios.rs`)

1. `scenario_text_roundtrip`
- API: `write_text`, `read_text`
- Goal: verify basic text read/write works on real OS clipboard.

2. `scenario_html_and_rtf_roundtrip`
- API: `write_html`, `read_html`, `write_rtf`, `read_rtf`
- Goal: verify formatted text transfer.

3. `scenario_image_write_bytes_and_read`
- API: `write_image_bytes`, `read_image`
- Goal: verify raw PNG bytes flow and image metadata output.

4. `scenario_image_write_path_and_read`
- API: `write_image`, `read_image`
- Goal: verify image file path input and clipboard image retrieval.

5. `scenario_files_and_video_files`
- API: `write_files`, `write_video_files`, `read_files`
- Goal: verify mixed file list and strict video-path validation.

6. `scenario_buffer_roundtrip`
- API: `write_buffer`, `read_buffer`
- Goal: verify arbitrary binary clipboard format support.

7. `scenario_snapshot_and_clear`
- API: `read_clipboard`, `clear`
- Goal: verify snapshot aggregation and clear behavior.

8. `scenario_available_formats_and_has_format`
- API: `available_formats`, `has_format`
- Goal: verify runtime format introspection.

## Run

```bash
cargo test
cargo test --test real_scenarios -- --ignored
```

## Notes

- Real scenarios are ignored by default to avoid CI instability.
- Platform clipboard security policies may affect behavior in sandboxed environments.
