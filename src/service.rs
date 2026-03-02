use crate::backend::{ClipboardBackend, NativeClipboardBackend};
use crate::constants::format::PNG_ALIASES;
use crate::error::{ClipboardError, Result};
use crate::models::{
    BufferPayload, ClipboardSnapshot, ReadFiles, ReadImage, ReadImageOptions,
    WriteImageBytesRequest, WriteImageRequest,
};
use crate::utils;
use clipboard_rs::common::RustImage;
use clipboard_rs::{ClipboardContent, ContentFormat, RustImageData};
use image::ImageFormat;
use std::ffi::OsStr;
use std::fs;
use std::io::Cursor;
use std::path::{Path, PathBuf};

pub struct ClipboardService {
    backend: Box<dyn ClipboardBackend>,
}

impl ClipboardService {
    pub fn new_native() -> Result<Self> {
        let backend = NativeClipboardBackend::new()?;
        Ok(Self {
            backend: Box::new(backend),
        })
    }

    pub fn with_backend(backend: Box<dyn ClipboardBackend>) -> Self {
        Self { backend }
    }

    pub fn available_formats(&self) -> Result<Vec<String>> {
        self.backend.available_formats()
    }

    pub fn has_text(&self) -> bool {
        self.backend.has(ContentFormat::Text)
    }

    pub fn has_rtf(&self) -> bool {
        self.backend.has(ContentFormat::Rtf)
    }

    pub fn has_html(&self) -> bool {
        self.backend.has(ContentFormat::Html)
    }

    pub fn has_image(&self) -> bool {
        self.backend.has(ContentFormat::Image)
    }

    pub fn has_files(&self) -> bool {
        self.backend.has(ContentFormat::Files)
    }

    pub fn has_format(&self, format: &str) -> Result<bool> {
        let target = format.trim();
        if target.is_empty() {
            return Err(ClipboardError::InvalidArgument(
                "format cannot be empty".to_string(),
            ));
        }
        let found = self
            .backend
            .available_formats()?
            .iter()
            .any(|current| current.eq_ignore_ascii_case(target));
        Ok(found)
    }

    pub fn read_text(&self) -> Result<String> {
        self.backend.get_text()
    }

    pub fn read_rtf(&self) -> Result<String> {
        self.backend.get_rich_text()
    }

    pub fn read_html(&self) -> Result<String> {
        self.backend.get_html()
    }

    pub fn read_buffer(&self, format: &str) -> Result<BufferPayload> {
        let format = normalize_format_name(format)?;
        let data = self.backend.get_buffer(&format)?;
        Ok(BufferPayload { format, data })
    }

    pub fn read_files(&self) -> Result<ReadFiles> {
        let raw_files = self.backend.get_files()?;
        let files = raw_files
            .iter()
            .map(|item| utils::to_file_item(&utils::normalize_clipboard_path(item)))
            .collect::<Vec<_>>();
        let size = files.iter().map(|item| item.size).sum();
        Ok(ReadFiles { files, size })
    }

    pub fn read_image(
        &self,
        options: Option<ReadImageOptions>,
        default_dir: &Path,
    ) -> Result<ReadImage> {
        let opts = options.unwrap_or_default();
        let image_bytes = if opts.prefer_raw_png.unwrap_or(false) {
            if let Some(raw) = self.try_read_raw_png()? {
                raw
            } else {
                self.read_image_as_png()?
            }
        } else {
            self.read_image_as_png()?
        };
        build_image_result(image_bytes, &opts, default_dir)
    }

    pub fn read_clipboard(
        &self,
        options: Option<ReadImageOptions>,
        default_dir: &Path,
    ) -> Result<ClipboardSnapshot> {
        let available_formats = self.available_formats().unwrap_or_default();
        let text = self.read_text().ok();
        let rtf = self.read_rtf().ok();
        let html = self.read_html().ok();
        let image = self.read_image(options, default_dir).ok();
        let files = self.read_files().ok();

        Ok(ClipboardSnapshot {
            available_formats,
            text,
            rtf,
            html,
            image,
            files,
        })
    }

    pub fn write_text(&self, content: String) -> Result<()> {
        self.backend.set(vec![ClipboardContent::Text(content)])
    }

    pub fn write_rtf(&self, content: String) -> Result<()> {
        self.backend.set(vec![
            ClipboardContent::Rtf(content.clone()),
            ClipboardContent::Text(content),
        ])
    }

    pub fn write_html(&self, content: String) -> Result<()> {
        self.backend.set(vec![
            ClipboardContent::Html(content.clone()),
            ClipboardContent::Text(content),
        ])
    }

    pub fn write_image(&self, request: WriteImageRequest) -> Result<()> {
        let (bytes, prefer_raw_png, also_standard, format_hint) =
            parse_image_write_request(request)?;
        self.write_image_with_fallback(bytes, format_hint.as_deref(), prefer_raw_png, also_standard)
    }

    pub fn write_image_bytes(&self, request: WriteImageBytesRequest) -> Result<()> {
        if request.bytes.is_empty() {
            return Err(ClipboardError::InvalidArgument(
                "image bytes cannot be empty".to_string(),
            ));
        }

        let format = request
            .format
            .unwrap_or_else(|| "image/png".to_string())
            .trim()
            .to_string();
        let fast_only = request.fast_only.unwrap_or(false);
        if is_image_format(&format) {
            return self.write_image_with_fallback(request.bytes, Some(&format), true, !fast_only);
        }
        self.backend.set_buffer(&format, request.bytes)
    }

    pub fn write_files(&self, files: Vec<String>) -> Result<()> {
        let files = sanitize_paths(files)?;
        self.backend.set(vec![ClipboardContent::Files(files)])
    }

    pub fn write_video_files(&self, files: Vec<String>) -> Result<()> {
        let files = sanitize_paths(files)?;
        if files.iter().any(|path| !is_video_path(path)) {
            return Err(ClipboardError::InvalidArgument(
                "all file paths must be video files".to_string(),
            ));
        }
        self.backend.set(vec![ClipboardContent::Files(files)])
    }

    pub fn write_buffer(&self, payload: BufferPayload) -> Result<()> {
        let format = normalize_format_name(&payload.format)?;
        self.backend.set_buffer(&format, payload.data)
    }

    pub fn clear(&self) -> Result<()> {
        self.backend.clear()
    }

    fn read_image_as_png(&self) -> Result<Vec<u8>> {
        let image = self.backend.get_image()?;
        image
            .to_png()
            .map(|buffer| buffer.get_bytes().to_vec())
            .map_err(|err| ClipboardError::Backend(err.to_string()))
    }

    fn try_read_raw_png(&self) -> Result<Option<Vec<u8>>> {
        for format in PNG_ALIASES {
            if let Ok(data) = self.backend.get_buffer(format) {
                if !data.is_empty() {
                    return Ok(Some(data));
                }
            }
        }
        Ok(None)
    }

    fn write_image_with_fallback(
        &self,
        image_bytes: Vec<u8>,
        format_hint: Option<&str>,
        prefer_raw_png: bool,
        also_standard: bool,
    ) -> Result<()> {
        if image_bytes.is_empty() {
            return Err(ClipboardError::InvalidArgument(
                "png bytes cannot be empty".to_string(),
            ));
        }
        let png_bytes = match ensure_png_bytes(image_bytes.clone()) {
            Ok(bytes) => bytes,
            Err(error) => {
                let hint = normalize_image_format_hint(format_hint);
                if let Some(image_format) = hint {
                    return self
                        .backend
                        .set(vec![ClipboardContent::Other(image_format, image_bytes)]);
                }
                return Err(error);
            }
        };

        if prefer_raw_png && !also_standard {
            let contents = PNG_ALIASES
                .iter()
                .map(|format| ClipboardContent::Other((*format).to_string(), png_bytes.clone()))
                .collect::<Vec<_>>();
            return self.backend.set(contents);
        }

        let image = RustImageData::from_bytes(&png_bytes).map_err(|err| {
            ClipboardError::InvalidArgument(format!("invalid image bytes: {err}"))
        })?;

        let mut contents = vec![ClipboardContent::Image(image)];
        contents.extend(
            PNG_ALIASES
                .iter()
                .map(|format| ClipboardContent::Other((*format).to_string(), png_bytes.clone())),
        );
        self.backend.set(contents)
    }
}

fn sanitize_paths(files: Vec<String>) -> Result<Vec<String>> {
    let files = files
        .iter()
        .map(|item| item.trim())
        .filter(|item| !item.is_empty())
        .map(ToOwned::to_owned)
        .collect::<Vec<_>>();
    if files.is_empty() {
        return Err(ClipboardError::InvalidArgument(
            "files cannot be empty".to_string(),
        ));
    }
    Ok(files)
}

fn parse_image_write_request(
    request: WriteImageRequest,
) -> Result<(Vec<u8>, bool, bool, Option<String>)> {
    let prefer_raw_png = request.prefer_raw_png.unwrap_or(true);
    let also_standard = request.also_set_standard_image.unwrap_or(true);

    match (request.path, request.bytes) {
        (Some(path), None) => {
            if !path.exists() {
                return Err(ClipboardError::NotFound(path.to_string_lossy().to_string()));
            }
            let bytes = fs::read(&path)?;
            if bytes.is_empty() {
                return Err(ClipboardError::InvalidArgument(
                    "image file is empty".to_string(),
                ));
            }
            let raw_png = path
                .extension()
                .and_then(OsStr::to_str)
                .map(|ext| ext.eq_ignore_ascii_case("png"))
                .unwrap_or(false);
            let format_hint = path
                .extension()
                .and_then(OsStr::to_str)
                .and_then(utils::mime_from_extension);
            Ok((bytes, prefer_raw_png && raw_png, also_standard, format_hint))
        }
        (None, Some(bytes)) => {
            if bytes.is_empty() {
                return Err(ClipboardError::InvalidArgument(
                    "image bytes cannot be empty".to_string(),
                ));
            }
            Ok((bytes, prefer_raw_png, also_standard, None))
        }
        (Some(_), Some(_)) => Err(ClipboardError::InvalidArgument(
            "provide either path or bytes for write_image, not both".to_string(),
        )),
        (None, None) => Err(ClipboardError::InvalidArgument(
            "path or bytes is required for write_image".to_string(),
        )),
    }
}

fn normalize_format_name(format: &str) -> Result<String> {
    let format = format.trim();
    if format.is_empty() {
        return Err(ClipboardError::InvalidArgument(
            "format cannot be empty".to_string(),
        ));
    }
    Ok(format.to_string())
}

fn is_image_format(format: &str) -> bool {
    let normalized = format.trim().to_ascii_lowercase();
    normalized.starts_with("image/")
        || matches!(
            normalized.as_str(),
            "png"
                | "jpg"
                | "jpeg"
                | "gif"
                | "webp"
                | "bmp"
                | "tif"
                | "tiff"
                | "ico"
                | "avif"
                | "heic"
                | "svg"
        )
}

fn normalize_image_format_hint(format_hint: Option<&str>) -> Option<String> {
    let hint = format_hint?.trim();
    if hint.is_empty() || !is_image_format(hint) {
        return None;
    }
    let normalized = if hint.contains('/') {
        hint.to_ascii_lowercase()
    } else {
        format!("image/{}", hint.to_ascii_lowercase())
    };
    Some(normalized)
}

fn ensure_png_bytes(bytes: Vec<u8>) -> Result<Vec<u8>> {
    if bytes.starts_with(&[137, 80, 78, 71, 13, 10, 26, 10]) {
        return Ok(bytes);
    }

    let image = image::load_from_memory(&bytes)
        .map_err(|err| ClipboardError::InvalidArgument(format!("invalid image bytes: {err}")))?;
    let mut png_bytes = Vec::new();
    image
        .write_to(&mut Cursor::new(&mut png_bytes), ImageFormat::Png)
        .map_err(|err| ClipboardError::Backend(format!("encode png failed: {err}")))?;
    Ok(png_bytes)
}

fn is_video_path(path: &str) -> bool {
    let ext = PathBuf::from(path)
        .extension()
        .and_then(OsStr::to_str)
        .unwrap_or_default()
        .to_ascii_lowercase();
    utils::is_video_extension(&ext)
}

fn build_image_result(
    bytes: Vec<u8>,
    options: &ReadImageOptions,
    default_dir: &Path,
) -> Result<ReadImage> {
    let (width, height) = if let Some(size) = parse_png_dimensions(&bytes) {
        size
    } else {
        let image = RustImageData::from_bytes(&bytes)
            .map_err(|err| ClipboardError::Backend(format!("decode image failed: {err}")))?;
        image.get_size()
    };

    let should_save = options.auto_save.unwrap_or(false) || options.save_to.is_some();
    let image_path = if should_save {
        let save_dir = options
            .save_to
            .clone()
            .unwrap_or_else(|| default_dir.to_path_buf());
        utils::ensure_dir(&save_dir)?;
        let file_hash = utils::hash_bytes(&bytes);
        let path = save_dir.join(format!("{file_hash}.png"));
        if !path.exists() {
            fs::write(&path, &bytes)?;
        }
        Some(path)
    } else {
        None
    };

    Ok(ReadImage {
        path: image_path,
        width,
        height,
        size: bytes.len() as u64,
        format: "image/png".to_string(),
        bytes: options.include_bytes.unwrap_or(false).then_some(bytes),
    })
}

fn parse_png_dimensions(bytes: &[u8]) -> Option<(u32, u32)> {
    const PNG_SIG: [u8; 8] = [137, 80, 78, 71, 13, 10, 26, 10];
    if bytes.len() < 24 || bytes[..8] != PNG_SIG {
        return None;
    }
    let width = u32::from_be_bytes([bytes[16], bytes[17], bytes[18], bytes[19]]);
    let height = u32::from_be_bytes([bytes[20], bytes[21], bytes[22], bytes[23]]);
    Some((width, height))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::backend::ClipboardBackend;
    use clipboard_rs::{ClipboardContent, ContentFormat, RustImageData};
    use image::{DynamicImage, ImageFormat, RgbaImage};
    use parking_lot::Mutex;
    use std::collections::BTreeMap;
    use std::io::Cursor;
    use tempfile::tempdir;

    struct MockBackend {
        text: Mutex<Option<String>>,
        rtf: Mutex<Option<String>>,
        html: Mutex<Option<String>>,
        files: Mutex<Vec<String>>,
        buffers: Mutex<BTreeMap<String, Vec<u8>>>,
        image: Mutex<Option<Vec<u8>>>,
        has_files: Mutex<bool>,
    }

    impl MockBackend {
        fn new() -> Self {
            Self {
                text: Mutex::new(None),
                rtf: Mutex::new(None),
                html: Mutex::new(None),
                files: Mutex::new(Vec::new()),
                buffers: Mutex::new(BTreeMap::new()),
                image: Mutex::new(None),
                has_files: Mutex::new(false),
            }
        }
    }

    impl ClipboardBackend for MockBackend {
        fn available_formats(&self) -> Result<Vec<String>> {
            Ok(vec!["text/plain".to_string(), "image/png".to_string()])
        }

        fn has(&self, format: ContentFormat) -> bool {
            match format {
                ContentFormat::Text => self.text.lock().is_some(),
                ContentFormat::Rtf => self.rtf.lock().is_some(),
                ContentFormat::Html => self.html.lock().is_some(),
                ContentFormat::Image => self.image.lock().is_some(),
                ContentFormat::Files => *self.has_files.lock(),
                ContentFormat::Other(_) => false,
            }
        }

        fn clear(&self) -> Result<()> {
            self.text.lock().take();
            self.rtf.lock().take();
            self.html.lock().take();
            self.files.lock().clear();
            self.buffers.lock().clear();
            self.image.lock().take();
            Ok(())
        }

        fn get_text(&self) -> Result<String> {
            self.text
                .lock()
                .clone()
                .ok_or_else(|| ClipboardError::Backend("no text".to_string()))
        }

        fn get_rich_text(&self) -> Result<String> {
            self.rtf
                .lock()
                .clone()
                .ok_or_else(|| ClipboardError::Backend("no rtf".to_string()))
        }

        fn get_html(&self) -> Result<String> {
            self.html
                .lock()
                .clone()
                .ok_or_else(|| ClipboardError::Backend("no html".to_string()))
        }

        fn get_image(&self) -> Result<RustImageData> {
            let bytes = self
                .image
                .lock()
                .clone()
                .ok_or_else(|| ClipboardError::Backend("no image".to_string()))?;
            RustImageData::from_bytes(&bytes)
                .map_err(|err| ClipboardError::Backend(format!("invalid image: {err}")))
        }

        fn get_files(&self) -> Result<Vec<String>> {
            Ok(self.files.lock().clone())
        }

        fn get_buffer(&self, format: &str) -> Result<Vec<u8>> {
            self.buffers
                .lock()
                .get(format)
                .cloned()
                .ok_or_else(|| ClipboardError::Backend("no buffer".to_string()))
        }

        fn set(&self, contents: Vec<ClipboardContent>) -> Result<()> {
            for content in contents {
                match content {
                    ClipboardContent::Text(v) => {
                        *self.text.lock() = Some(v);
                    }
                    ClipboardContent::Rtf(v) => {
                        *self.rtf.lock() = Some(v);
                    }
                    ClipboardContent::Html(v) => {
                        *self.html.lock() = Some(v);
                    }
                    ClipboardContent::Image(image) => {
                        let png = image
                            .to_png()
                            .map_err(|err| ClipboardError::Backend(err.to_string()))?;
                        *self.image.lock() = Some(png.get_bytes().to_vec());
                    }
                    ClipboardContent::Files(v) => {
                        *self.files.lock() = v;
                        *self.has_files.lock() = true;
                    }
                    ClipboardContent::Other(fmt, data) => {
                        self.buffers.lock().insert(fmt, data);
                    }
                }
            }
            Ok(())
        }

        fn set_buffer(&self, format: &str, buffer: Vec<u8>) -> Result<()> {
            self.buffers.lock().insert(format.to_string(), buffer);
            Ok(())
        }
    }

    fn tiny_png() -> Vec<u8> {
        let image =
            RgbaImage::from_raw(1, 1, vec![255, 0, 0, 255]).expect("create tiny rgba image");
        let mut bytes = Vec::new();
        DynamicImage::ImageRgba8(image)
            .write_to(&mut Cursor::new(&mut bytes), ImageFormat::Png)
            .expect("encode tiny png");
        bytes
    }

    fn tiny_webp() -> Vec<u8> {
        vec![
            82, 73, 70, 70, 34, 0, 0, 0, 87, 69, 66, 80, 86, 80, 56, 32, 22, 0, 0, 0, 48, 1, 0,
            157, 1, 42, 1, 0, 1, 0, 1, 64, 38, 37, 164, 0, 3, 112, 0, 254, 251, 148, 0, 0,
        ]
    }

    #[test]
    fn write_and_read_text_works() {
        let backend = MockBackend::new();
        let service = ClipboardService::with_backend(Box::new(backend));
        service.write_text("hello".to_string()).unwrap();
        assert_eq!(service.read_text().unwrap(), "hello");
    }

    #[test]
    fn has_format_validates_input() {
        let backend = MockBackend::new();
        let service = ClipboardService::with_backend(Box::new(backend));
        assert!(service.has_format("image/png").unwrap());
        assert!(service.has_format("  ").is_err());
    }

    #[test]
    fn write_buffer_rejects_empty_format() {
        let backend = MockBackend::new();
        let service = ClipboardService::with_backend(Box::new(backend));
        let err = service
            .write_buffer(BufferPayload {
                format: " ".to_string(),
                data: vec![1],
            })
            .unwrap_err();
        assert!(matches!(err, ClipboardError::InvalidArgument(_)));
    }

    #[test]
    fn read_image_with_raw_png_path() {
        let backend = MockBackend::new();
        backend
            .set_buffer(PNG_ALIASES[0], tiny_png())
            .expect("set image/png buffer for test");
        let service = ClipboardService::with_backend(Box::new(backend));

        let out_dir = tempdir().unwrap();
        let image = service
            .read_image(
                Some(ReadImageOptions {
                    include_bytes: Some(true),
                    save_to: Some(out_dir.path().to_path_buf()),
                    auto_save: Some(true),
                    prefer_raw_png: Some(true),
                }),
                out_dir.path(),
            )
            .unwrap();

        assert_eq!(image.width, 1);
        assert_eq!(image.height, 1);
        assert!(image.path.expect("path should exist").exists());
        assert!(image.bytes.expect("bytes should exist").len() > 10);
    }

    #[test]
    fn write_video_files_validates_extensions() {
        let backend = MockBackend::new();
        let service = ClipboardService::with_backend(Box::new(backend));
        assert!(service
            .write_video_files(vec!["/tmp/a.txt".to_string()])
            .is_err());
        assert!(service
            .write_video_files(vec!["/tmp/a.mp4".to_string(), "/tmp/b.mov".to_string()])
            .is_ok());
    }

    #[test]
    fn write_image_request_from_path_works() {
        let backend = MockBackend::new();
        let service = ClipboardService::with_backend(Box::new(backend));

        let dir = tempdir().unwrap();
        let file = dir.path().join("tiny.png");
        fs::write(&file, tiny_png()).unwrap();

        service
            .write_image(WriteImageRequest {
                path: Some(file),
                bytes: None,
                prefer_raw_png: Some(true),
                also_set_standard_image: Some(true),
            })
            .unwrap();

        assert!(service.has_image());
    }

    #[test]
    fn write_image_request_from_webp_path_works() {
        let backend = MockBackend::new();
        let service = ClipboardService::with_backend(Box::new(backend));

        let dir = tempdir().unwrap();
        let file = dir.path().join("tiny.webp");
        fs::write(&file, tiny_webp()).unwrap();

        service
            .write_image(WriteImageRequest {
                path: Some(file),
                bytes: None,
                prefer_raw_png: Some(true),
                also_set_standard_image: Some(true),
            })
            .unwrap();

        assert!(service.read_buffer("image/webp").is_ok() || service.has_image());
    }

    #[test]
    fn write_image_bytes_webp_works() {
        let backend = MockBackend::new();
        let service = ClipboardService::with_backend(Box::new(backend));

        service
            .write_image_bytes(WriteImageBytesRequest {
                bytes: tiny_webp(),
                format: Some("image/webp".to_string()),
                fast_only: Some(false),
            })
            .unwrap();

        assert!(service.read_buffer("image/webp").is_ok() || service.has_image());
    }

    #[test]
    fn write_image_bytes_webp_fallback_raw_buffer_works() {
        let backend = MockBackend::new();
        let service = ClipboardService::with_backend(Box::new(backend));
        let invalid_webp = b"RIFF\x0A\0\0\0WEBPVP8 \x02\0\0\0\0\0".to_vec();

        service
            .write_image_bytes(WriteImageBytesRequest {
                bytes: invalid_webp.clone(),
                format: Some("image/webp".to_string()),
                fast_only: Some(false),
            })
            .unwrap();

        let payload = service.read_buffer("image/webp").unwrap();
        assert_eq!(payload.data, invalid_webp);
    }

    #[test]
    fn read_files_maps_file_uri() {
        let backend = MockBackend::new();
        backend
            .set(vec![ClipboardContent::Files(vec![
                "file:///tmp/a%20b.mp4".to_string()
            ])])
            .unwrap();
        let service = ClipboardService::with_backend(Box::new(backend));
        let files = service.read_files().unwrap();
        assert_eq!(files.files.len(), 1);
        assert!(files.files[0].path.contains("a b.mp4"));
    }
}
