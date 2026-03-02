use crate::error::{ClipboardError, Result};
use crate::models::{
    BufferPayload, ClipboardSnapshot, ReadFiles, ReadImage, ReadImageOptions,
    WriteImageBytesRequest, WriteImageRequest,
};
use serde::de::DeserializeOwned;
use std::path::PathBuf;
use tauri::{plugin::PluginApi, AppHandle, Runtime};

pub fn init<R: Runtime, C: DeserializeOwned>(
    _app: &AppHandle<R>,
    _api: PluginApi<R, C>,
) -> Result<ClipboardPro<R>> {
    Ok(ClipboardPro {
        _runtime: std::marker::PhantomData,
    })
}

/// EN: Mobile stub implementation.
/// CN: 移动端占位实现。
pub struct ClipboardPro<R: Runtime> {
    _runtime: std::marker::PhantomData<R>,
}

impl<R: Runtime> ClipboardPro<R> {
    pub fn start_watch(&self) -> Result<()> {
        Err(unsupported())
    }

    pub fn stop_watch(&self) -> Result<()> {
        Err(unsupported())
    }

    pub fn available_formats(&self) -> Result<Vec<String>> {
        Err(unsupported())
    }

    pub fn has_text(&self) -> bool {
        false
    }

    pub fn has_rtf(&self) -> bool {
        false
    }

    pub fn has_html(&self) -> bool {
        false
    }

    pub fn has_image(&self) -> bool {
        false
    }

    pub fn has_files(&self) -> bool {
        false
    }

    pub fn has_format(&self, _format: &str) -> Result<bool> {
        Err(unsupported())
    }

    pub fn read_text(&self) -> Result<String> {
        Err(unsupported())
    }

    pub fn read_rtf(&self) -> Result<String> {
        Err(unsupported())
    }

    pub fn read_html(&self) -> Result<String> {
        Err(unsupported())
    }

    pub fn read_image(&self, _options: Option<ReadImageOptions>) -> Result<ReadImage> {
        Err(unsupported())
    }

    pub fn read_files(&self) -> Result<ReadFiles> {
        Err(unsupported())
    }

    pub fn read_buffer(&self, _format: &str) -> Result<BufferPayload> {
        Err(unsupported())
    }

    pub fn read_clipboard(&self, _options: Option<ReadImageOptions>) -> Result<ClipboardSnapshot> {
        Err(unsupported())
    }

    pub fn write_text(&self, _content: String) -> Result<()> {
        Err(unsupported())
    }

    pub fn write_rtf(&self, _content: String) -> Result<()> {
        Err(unsupported())
    }

    pub fn write_html(&self, _content: String) -> Result<()> {
        Err(unsupported())
    }

    pub fn write_image(&self, _request: WriteImageRequest) -> Result<()> {
        Err(unsupported())
    }

    pub fn write_image_bytes(&self, _request: WriteImageBytesRequest) -> Result<()> {
        Err(unsupported())
    }

    pub fn write_files(&self, _files: Vec<String>) -> Result<()> {
        Err(unsupported())
    }

    pub fn write_video_files(&self, _files: Vec<String>) -> Result<()> {
        Err(unsupported())
    }

    pub fn write_buffer(&self, _payload: BufferPayload) -> Result<()> {
        Err(unsupported())
    }

    pub fn clear(&self) -> Result<()> {
        Err(unsupported())
    }

    pub fn get_file_path(&self) -> Result<PathBuf> {
        Err(unsupported())
    }
}

fn unsupported() -> ClipboardError {
    ClipboardError::Unsupported("mobile runtime is not implemented yet".to_string())
}
