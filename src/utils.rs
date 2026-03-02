use crate::models::{FileItem, FileKind};
use std::collections::hash_map::DefaultHasher;
use std::ffi::OsStr;
use std::fs;
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};

pub fn hash_bytes(bytes: &[u8]) -> u64 {
    let mut hasher = DefaultHasher::new();
    bytes.len().hash(&mut hasher);
    if bytes.len() <= 8192 {
        bytes.hash(&mut hasher);
    } else {
        bytes[..4096].hash(&mut hasher);
        bytes[bytes.len() - 4096..].hash(&mut hasher);
    }
    hasher.finish()
}

pub fn ensure_dir(path: &Path) -> std::io::Result<()> {
    if path.exists() {
        return Ok(());
    }
    fs::create_dir_all(path)
}

pub fn normalize_clipboard_path(raw: &str) -> String {
    let trimmed = raw.trim();
    if !trimmed.starts_with("file://") {
        return trimmed.to_string();
    }
    let without_scheme = trimmed.trim_start_matches("file://");
    let decoded = percent_decode(without_scheme);
    #[cfg(windows)]
    {
        if decoded.starts_with('/') && decoded.chars().nth(2) == Some(':') {
            return decoded.trim_start_matches('/').to_string();
        }
    }
    decoded
}

pub fn percent_decode(input: &str) -> String {
    let bytes = input.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut i = 0usize;
    while i < bytes.len() {
        if bytes[i] == b'%' && i + 2 < bytes.len() {
            let h1 = bytes[i + 1] as char;
            let h2 = bytes[i + 2] as char;
            if let (Some(a), Some(b)) = (h1.to_digit(16), h2.to_digit(16)) {
                out.push(((a << 4) as u8) | (b as u8));
                i += 3;
                continue;
            }
        }
        out.push(bytes[i]);
        i += 1;
    }
    String::from_utf8_lossy(&out).to_string()
}

pub fn to_file_item(path: &str) -> FileItem {
    let path_buf = PathBuf::from(path);
    let metadata = fs::metadata(&path_buf).ok();
    let is_dir = metadata.as_ref().is_some_and(|m| m.is_dir());
    let size = metadata.as_ref().map_or(0, |m| m.len());
    let kind = detect_file_kind(&path_buf, is_dir);
    let mime = mime_from_path(&path_buf);

    FileItem {
        path: path.to_string(),
        size,
        is_dir,
        kind,
        mime,
    }
}

pub fn detect_file_kind(path: &Path, is_dir: bool) -> FileKind {
    if is_dir {
        return FileKind::Directory;
    }
    let ext = path.extension().and_then(OsStr::to_str).unwrap_or_default();
    let ext = ext.to_ascii_lowercase();
    if ext.is_empty() {
        return FileKind::Unknown;
    }
    if is_image_extension(&ext) {
        return FileKind::Image;
    }
    if is_video_extension(&ext) {
        return FileKind::Video;
    }
    FileKind::File
}

pub fn mime_from_path(path: &Path) -> Option<String> {
    let ext = path.extension().and_then(OsStr::to_str)?;
    mime_from_extension(ext)
}

pub fn mime_from_extension(ext: &str) -> Option<String> {
    let ext = ext.to_ascii_lowercase();
    let mime = match ext.as_str() {
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "gif" => "image/gif",
        "webp" => "image/webp",
        "bmp" => "image/bmp",
        "tif" | "tiff" => "image/tiff",
        "ico" => "image/x-icon",
        "heic" => "image/heic",
        "avif" => "image/avif",
        "svg" => "image/svg+xml",
        "mp4" => "video/mp4",
        "mov" => "video/quicktime",
        "mkv" => "video/x-matroska",
        "avi" => "video/x-msvideo",
        "webm" => "video/webm",
        "m4v" => "video/x-m4v",
        "mpeg" | "mpg" => "video/mpeg",
        "flv" => "video/x-flv",
        "wmv" => "video/x-ms-wmv",
        "3gp" => "video/3gpp",
        "ts" => "video/mp2t",
        _ => return None,
    };
    Some(mime.to_string())
}

pub fn is_video_extension(ext: &str) -> bool {
    matches!(
        ext,
        "mp4"
            | "mov"
            | "mkv"
            | "avi"
            | "webm"
            | "m4v"
            | "mpeg"
            | "mpg"
            | "flv"
            | "wmv"
            | "3gp"
            | "ts"
    )
}

pub fn is_image_extension(ext: &str) -> bool {
    matches!(
        ext,
        "png"
            | "jpg"
            | "jpeg"
            | "gif"
            | "webp"
            | "bmp"
            | "tif"
            | "tiff"
            | "ico"
            | "heic"
            | "avif"
            | "svg"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_percent_decode() {
        assert_eq!(percent_decode("/tmp/a%20b.txt"), "/tmp/a b.txt");
        assert_eq!(percent_decode("plain"), "plain");
    }

    #[test]
    fn test_normalize_clipboard_path() {
        assert_eq!(
            normalize_clipboard_path("file:///tmp/a%20b.txt"),
            "/tmp/a b.txt"
        );
        assert_eq!(normalize_clipboard_path("/tmp/plain"), "/tmp/plain");
    }

    #[test]
    fn test_kind_detection() {
        assert!(matches!(
            detect_file_kind(Path::new("a.mp4"), false),
            FileKind::Video
        ));
        assert!(matches!(
            detect_file_kind(Path::new("a.png"), false),
            FileKind::Image
        ));
        assert!(matches!(
            detect_file_kind(Path::new("a.abc"), false),
            FileKind::File
        ));
    }

    #[test]
    fn test_mime_mapping() {
        assert_eq!(mime_from_extension("MP4"), Some("video/mp4".to_string()));
        assert_eq!(mime_from_extension("png"), Some("image/png".to_string()));
        assert_eq!(mime_from_extension("unknown"), None);
    }
}
