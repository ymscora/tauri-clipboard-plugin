use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, Runtime,
};

pub use error::{ApiError, ApiResult, ClipboardError, Result};
pub use models::*;
pub use service::ClipboardService;

mod backend;
mod commands;
mod constants;
mod error;
mod models;
mod service;
mod utils;

#[cfg(desktop)]
mod desktop;
#[cfg(mobile)]
mod mobile;

#[cfg(desktop)]
use desktop::ClipboardPro;
#[cfg(mobile)]
use mobile::ClipboardPro;

/// EN: Extension trait for accessing plugin API from `App`/`AppHandle`/`Window`.
/// CN: 给 `App`/`AppHandle`/`Window` 提供插件访问入口。
pub trait ClipboardExt<R: Runtime> {
    fn clipboard_pro(&self) -> &ClipboardPro<R>;
}

impl<R: Runtime, T: Manager<R>> ClipboardExt<R> for T {
    fn clipboard_pro(&self) -> &ClipboardPro<R> {
        self.state::<ClipboardPro<R>>().inner()
    }
}

/// EN: Initialize plugin.
/// CN: 初始化插件。
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new(constants::plugin::NAME)
        .invoke_handler(tauri::generate_handler![
            commands::start_watch,
            commands::stop_watch,
            commands::available_formats,
            commands::has_text,
            commands::has_rtf,
            commands::has_html,
            commands::has_image,
            commands::has_files,
            commands::has_format,
            commands::read_text,
            commands::read_rtf,
            commands::read_html,
            commands::read_image,
            commands::read_files,
            commands::read_buffer,
            commands::read_clipboard,
            commands::write_text,
            commands::write_rtf,
            commands::write_html,
            commands::write_image,
            commands::write_image_bytes,
            commands::write_files,
            commands::write_video_files,
            commands::write_buffer,
            commands::clear,
            commands::get_file_path,
        ])
        .setup(|app, api| {
            #[cfg(mobile)]
            let clipboard_pro = mobile::init(app, api)?;

            #[cfg(desktop)]
            let clipboard_pro = desktop::init(app, api)?;

            app.manage(clipboard_pro);
            Ok(())
        })
        .build()
}
