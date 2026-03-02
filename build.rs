const COMMANDS: &[&str] = &[
    "start_watch",
    "stop_watch",
    "available_formats",
    "has_text",
    "has_rtf",
    "has_html",
    "has_image",
    "has_files",
    "has_format",
    "read_text",
    "read_rtf",
    "read_html",
    "read_image",
    "read_files",
    "read_buffer",
    "read_clipboard",
    "write_text",
    "write_rtf",
    "write_html",
    "write_image",
    "write_image_bytes",
    "write_files",
    "write_video_files",
    "write_buffer",
    "clear",
    "get_file_path",
];

fn main() {
    tauri_plugin::Builder::new(COMMANDS).build();
}
