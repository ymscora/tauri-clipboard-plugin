use crate::constants;
use crate::error::{ClipboardError, Result};
use crate::models::{
    BufferPayload, ClipboardSnapshot, ReadFiles, ReadImage, ReadImageOptions,
    WriteImageBytesRequest, WriteImageRequest,
};
use crate::service::ClipboardService;
use clipboard_rs::{ClipboardHandler, ClipboardWatcher, ClipboardWatcherContext, WatcherShutdown};
use parking_lot::Mutex;
use serde::de::DeserializeOwned;
use std::sync::Arc;
use std::thread;
use tauri::{plugin::PluginApi, AppHandle, Emitter, Manager, Runtime};

pub(crate) struct ClipboardRuntimeState {
    service: Arc<ClipboardService>,
    watcher_shutdown: Mutex<Option<WatcherShutdown>>,
}

pub fn init<R: Runtime, C: DeserializeOwned>(
    app: &AppHandle<R>,
    _api: PluginApi<R, C>,
) -> Result<ClipboardPro<R>> {
    let service = ClipboardService::new_native()?;
    Ok(ClipboardPro {
        app: app.clone(),
        state: Arc::new(ClipboardRuntimeState {
            service: Arc::new(service),
            watcher_shutdown: Mutex::new(None),
        }),
    })
}

/// EN: Access to high-performance clipboard operations.
/// CN: 高性能剪贴板能力入口。
pub struct ClipboardPro<R: Runtime> {
    app: AppHandle<R>,
    state: Arc<ClipboardRuntimeState>,
}

struct ClipboardWatchHandler<R: Runtime> {
    app: AppHandle<R>,
}

impl<R: Runtime> ClipboardHandler for ClipboardWatchHandler<R> {
    fn on_clipboard_change(&mut self) {
        let _ = self.app.emit(constants::event::CLIPBOARD_CHANGE, ());
    }
}

impl<R: Runtime> ClipboardPro<R> {
    pub fn start_watch(&self) -> Result<()> {
        let mut shutdown_guard = self.state.watcher_shutdown.lock();
        if shutdown_guard.is_some() {
            return Ok(());
        }

        let mut watcher = ClipboardWatcherContext::new()
            .map_err(|err| ClipboardError::Backend(format!("create watcher failed: {err}")))?;
        let shutdown = watcher
            .add_handler(ClipboardWatchHandler {
                app: self.app.clone(),
            })
            .get_shutdown_channel();

        *shutdown_guard = Some(shutdown);
        thread::spawn(move || {
            watcher.start_watch();
        });
        Ok(())
    }

    pub fn stop_watch(&self) -> Result<()> {
        let mut shutdown_guard = self.state.watcher_shutdown.lock();
        if let Some(shutdown) = shutdown_guard.take() {
            shutdown.stop();
        }
        Ok(())
    }

    pub fn available_formats(&self) -> Result<Vec<String>> {
        self.state.service.available_formats()
    }

    pub fn has_text(&self) -> bool {
        self.state.service.has_text()
    }

    pub fn has_rtf(&self) -> bool {
        self.state.service.has_rtf()
    }

    pub fn has_html(&self) -> bool {
        self.state.service.has_html()
    }

    pub fn has_image(&self) -> bool {
        self.state.service.has_image()
    }

    pub fn has_files(&self) -> bool {
        self.state.service.has_files()
    }

    pub fn has_format(&self, format: &str) -> Result<bool> {
        self.state.service.has_format(format)
    }

    pub fn read_text(&self) -> Result<String> {
        self.state.service.read_text()
    }

    pub fn read_rtf(&self) -> Result<String> {
        self.state.service.read_rtf()
    }

    pub fn read_html(&self) -> Result<String> {
        self.state.service.read_html()
    }

    pub fn read_image(&self, options: Option<ReadImageOptions>) -> Result<ReadImage> {
        let default_dir = self.get_file_path()?;
        self.state.service.read_image(options, &default_dir)
    }

    pub fn read_files(&self) -> Result<ReadFiles> {
        self.state.service.read_files()
    }

    pub fn read_buffer(&self, format: &str) -> Result<BufferPayload> {
        self.state.service.read_buffer(format)
    }

    pub fn read_clipboard(&self, options: Option<ReadImageOptions>) -> Result<ClipboardSnapshot> {
        let default_dir = self.get_file_path()?;
        self.state.service.read_clipboard(options, &default_dir)
    }

    pub fn write_text(&self, content: String) -> Result<()> {
        self.state.service.write_text(content)
    }

    pub fn write_rtf(&self, content: String) -> Result<()> {
        self.state.service.write_rtf(content)
    }

    pub fn write_html(&self, content: String) -> Result<()> {
        self.state.service.write_html(content)
    }

    pub fn write_image(&self, request: WriteImageRequest) -> Result<()> {
        self.state.service.write_image(request)
    }

    pub fn write_image_bytes(&self, request: WriteImageBytesRequest) -> Result<()> {
        self.state.service.write_image_bytes(request)
    }

    pub fn write_files(&self, files: Vec<String>) -> Result<()> {
        self.state.service.write_files(files)
    }

    pub fn write_video_files(&self, files: Vec<String>) -> Result<()> {
        self.state.service.write_video_files(files)
    }

    pub fn write_buffer(&self, payload: BufferPayload) -> Result<()> {
        self.state.service.write_buffer(payload)
    }

    pub fn clear(&self) -> Result<()> {
        self.state.service.clear()
    }

    pub fn get_file_path(&self) -> Result<std::path::PathBuf> {
        let app_data_dir = self.app.path().app_data_dir().map_err(|err| {
            ClipboardError::Backend(format!("resolve app_data_dir failed: {err}"))
        })?;

        let final_path = app_data_dir
            .join(constants::plugin::NAME)
            .join(constants::plugin::FILE_DIR);

        crate::utils::ensure_dir(&final_path)?;
        Ok(final_path)
    }
}
