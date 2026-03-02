use crate::models::{
    BufferPayload, ClipboardSnapshot, ReadFiles, ReadImage, ReadImageOptions,
    WriteImageBytesRequest, WriteImageRequest,
};
use crate::{ApiResult, ClipboardExt};
use std::path::PathBuf;
use tauri::{command, AppHandle, Runtime};

#[command]
pub(crate) async fn start_watch<R: Runtime>(app: AppHandle<R>) -> ApiResult<()> {
    app.clipboard_pro().start_watch().map_err(Into::into)
}

#[command]
pub(crate) async fn stop_watch<R: Runtime>(app: AppHandle<R>) -> ApiResult<()> {
    app.clipboard_pro().stop_watch().map_err(Into::into)
}

#[command]
pub(crate) async fn available_formats<R: Runtime>(app: AppHandle<R>) -> ApiResult<Vec<String>> {
    app.clipboard_pro().available_formats().map_err(Into::into)
}

#[command]
pub(crate) async fn has_text<R: Runtime>(app: AppHandle<R>) -> ApiResult<bool> {
    Ok(app.clipboard_pro().has_text())
}

#[command]
pub(crate) async fn has_rtf<R: Runtime>(app: AppHandle<R>) -> ApiResult<bool> {
    Ok(app.clipboard_pro().has_rtf())
}

#[command]
pub(crate) async fn has_html<R: Runtime>(app: AppHandle<R>) -> ApiResult<bool> {
    Ok(app.clipboard_pro().has_html())
}

#[command]
pub(crate) async fn has_image<R: Runtime>(app: AppHandle<R>) -> ApiResult<bool> {
    Ok(app.clipboard_pro().has_image())
}

#[command]
pub(crate) async fn has_files<R: Runtime>(app: AppHandle<R>) -> ApiResult<bool> {
    Ok(app.clipboard_pro().has_files())
}

#[command]
pub(crate) async fn has_format<R: Runtime>(app: AppHandle<R>, format: String) -> ApiResult<bool> {
    app.clipboard_pro().has_format(&format).map_err(Into::into)
}

#[command]
pub(crate) async fn read_text<R: Runtime>(app: AppHandle<R>) -> ApiResult<String> {
    app.clipboard_pro().read_text().map_err(Into::into)
}

#[command]
pub(crate) async fn read_rtf<R: Runtime>(app: AppHandle<R>) -> ApiResult<String> {
    app.clipboard_pro().read_rtf().map_err(Into::into)
}

#[command]
pub(crate) async fn read_html<R: Runtime>(app: AppHandle<R>) -> ApiResult<String> {
    app.clipboard_pro().read_html().map_err(Into::into)
}

#[command]
pub(crate) async fn read_image<R: Runtime>(
    app: AppHandle<R>,
    options: Option<ReadImageOptions>,
) -> ApiResult<ReadImage> {
    app.clipboard_pro().read_image(options).map_err(Into::into)
}

#[command]
pub(crate) async fn read_files<R: Runtime>(app: AppHandle<R>) -> ApiResult<ReadFiles> {
    app.clipboard_pro().read_files().map_err(Into::into)
}

#[command]
pub(crate) async fn read_buffer<R: Runtime>(
    app: AppHandle<R>,
    format: String,
) -> ApiResult<BufferPayload> {
    app.clipboard_pro().read_buffer(&format).map_err(Into::into)
}

#[command]
pub(crate) async fn read_clipboard<R: Runtime>(
    app: AppHandle<R>,
    options: Option<ReadImageOptions>,
) -> ApiResult<ClipboardSnapshot> {
    app.clipboard_pro()
        .read_clipboard(options)
        .map_err(Into::into)
}

#[command]
pub(crate) async fn write_text<R: Runtime>(app: AppHandle<R>, content: String) -> ApiResult<()> {
    app.clipboard_pro().write_text(content).map_err(Into::into)
}

#[command]
pub(crate) async fn write_rtf<R: Runtime>(app: AppHandle<R>, content: String) -> ApiResult<()> {
    app.clipboard_pro().write_rtf(content).map_err(Into::into)
}

#[command]
pub(crate) async fn write_html<R: Runtime>(app: AppHandle<R>, content: String) -> ApiResult<()> {
    app.clipboard_pro().write_html(content).map_err(Into::into)
}

#[command]
pub(crate) async fn write_image<R: Runtime>(
    app: AppHandle<R>,
    request: WriteImageRequest,
) -> ApiResult<()> {
    app.clipboard_pro().write_image(request).map_err(Into::into)
}

#[command]
pub(crate) async fn write_image_bytes<R: Runtime>(
    app: AppHandle<R>,
    request: WriteImageBytesRequest,
) -> ApiResult<()> {
    app.clipboard_pro()
        .write_image_bytes(request)
        .map_err(Into::into)
}

#[command]
pub(crate) async fn write_files<R: Runtime>(
    app: AppHandle<R>,
    files_path: Vec<String>,
) -> ApiResult<()> {
    app.clipboard_pro()
        .write_files(files_path)
        .map_err(Into::into)
}

#[command]
pub(crate) async fn write_video_files<R: Runtime>(
    app: AppHandle<R>,
    files_path: Vec<String>,
) -> ApiResult<()> {
    app.clipboard_pro()
        .write_video_files(files_path)
        .map_err(Into::into)
}

#[command]
pub(crate) async fn write_buffer<R: Runtime>(
    app: AppHandle<R>,
    payload: BufferPayload,
) -> ApiResult<()> {
    app.clipboard_pro()
        .write_buffer(payload)
        .map_err(Into::into)
}

#[command]
pub(crate) async fn clear<R: Runtime>(app: AppHandle<R>) -> ApiResult<()> {
    app.clipboard_pro().clear().map_err(Into::into)
}

#[command]
pub(crate) async fn get_file_path<R: Runtime>(app: AppHandle<R>) -> ApiResult<PathBuf> {
    app.clipboard_pro().get_file_path().map_err(Into::into)
}
