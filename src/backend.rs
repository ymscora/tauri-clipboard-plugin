use crate::error::{ClipboardError, Result};
use clipboard_rs::{Clipboard, ClipboardContent, ClipboardContext, ContentFormat, RustImageData};
use parking_lot::Mutex;

pub trait ClipboardBackend: Send + Sync {
    fn available_formats(&self) -> Result<Vec<String>>;
    fn has(&self, format: ContentFormat) -> bool;
    fn clear(&self) -> Result<()>;

    fn get_text(&self) -> Result<String>;
    fn get_rich_text(&self) -> Result<String>;
    fn get_html(&self) -> Result<String>;
    fn get_image(&self) -> Result<RustImageData>;
    fn get_files(&self) -> Result<Vec<String>>;
    fn get_buffer(&self, format: &str) -> Result<Vec<u8>>;

    fn set(&self, contents: Vec<ClipboardContent>) -> Result<()>;
    fn set_buffer(&self, format: &str, buffer: Vec<u8>) -> Result<()>;
}

pub struct NativeClipboardBackend {
    ctx: Mutex<ClipboardContext>,
}

impl NativeClipboardBackend {
    pub fn new() -> Result<Self> {
        let ctx = ClipboardContext::new().map_err(map_backend_error)?;
        Ok(Self {
            ctx: Mutex::new(ctx),
        })
    }
}

impl ClipboardBackend for NativeClipboardBackend {
    fn available_formats(&self) -> Result<Vec<String>> {
        self.ctx
            .lock()
            .available_formats()
            .map_err(map_backend_error)
    }

    fn has(&self, format: ContentFormat) -> bool {
        self.ctx.lock().has(format)
    }

    fn clear(&self) -> Result<()> {
        self.ctx.lock().clear().map_err(map_backend_error)
    }

    fn get_text(&self) -> Result<String> {
        self.ctx.lock().get_text().map_err(map_backend_error)
    }

    fn get_rich_text(&self) -> Result<String> {
        self.ctx.lock().get_rich_text().map_err(map_backend_error)
    }

    fn get_html(&self) -> Result<String> {
        self.ctx.lock().get_html().map_err(map_backend_error)
    }

    fn get_image(&self) -> Result<RustImageData> {
        self.ctx.lock().get_image().map_err(map_backend_error)
    }

    fn get_files(&self) -> Result<Vec<String>> {
        self.ctx.lock().get_files().map_err(map_backend_error)
    }

    fn get_buffer(&self, format: &str) -> Result<Vec<u8>> {
        self.ctx
            .lock()
            .get_buffer(format)
            .map_err(map_backend_error)
    }

    fn set(&self, contents: Vec<ClipboardContent>) -> Result<()> {
        self.ctx.lock().set(contents).map_err(map_backend_error)
    }

    fn set_buffer(&self, format: &str, buffer: Vec<u8>) -> Result<()> {
        self.ctx
            .lock()
            .set_buffer(format, buffer)
            .map_err(map_backend_error)
    }
}

fn map_backend_error<E: std::fmt::Display>(error: E) -> ClipboardError {
    ClipboardError::Backend(error.to_string())
}
