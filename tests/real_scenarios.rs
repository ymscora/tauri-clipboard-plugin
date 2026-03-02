#![cfg(desktop)]

use image::{DynamicImage, ImageFormat, RgbaImage};
use std::fs;
use std::io::Cursor;
use tauri_plugin_clipboard_pro::{
    BufferPayload, ClipboardService, ReadImageOptions, WriteImageBytesRequest, WriteImageRequest,
};
use tempfile::tempdir;

fn tiny_png() -> Vec<u8> {
    let image = RgbaImage::from_raw(1, 1, vec![255, 0, 0, 255]).expect("create tiny rgba image");
    let mut bytes = Vec::new();
    DynamicImage::ImageRgba8(image)
        .write_to(&mut Cursor::new(&mut bytes), ImageFormat::Png)
        .expect("encode tiny png");
    bytes
}

#[test]
#[ignore = "Requires real OS clipboard"]
fn scenario_text_roundtrip() {
    let service = ClipboardService::new_native().unwrap();
    service
        .write_text("hello from scenario".to_string())
        .unwrap();
    let got = service.read_text().unwrap();
    assert_eq!(got, "hello from scenario");
}

#[test]
#[ignore = "Requires real OS clipboard"]
fn scenario_html_and_rtf_roundtrip() {
    let service = ClipboardService::new_native().unwrap();
    service.write_html("<b>hello</b>".to_string()).unwrap();
    assert!(service.read_html().unwrap().contains("hello"));

    service
        .write_rtf("{\\rtf1\\ansi hello rtf}".to_string())
        .unwrap();
    assert!(service.read_rtf().unwrap().contains("rtf"));
}

#[test]
#[ignore = "Requires real OS clipboard"]
fn scenario_image_write_bytes_and_read() {
    let service = ClipboardService::new_native().unwrap();
    service
        .write_image_bytes(WriteImageBytesRequest {
            bytes: tiny_png(),
            format: Some("image/png".to_string()),
            fast_only: Some(false),
        })
        .unwrap();

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
    assert!(image.path.unwrap().exists());
}

#[test]
#[ignore = "Requires real OS clipboard"]
fn scenario_image_write_path_and_read() {
    let service = ClipboardService::new_native().unwrap();
    let dir = tempdir().unwrap();
    let image_path = dir.path().join("tiny.png");
    fs::write(&image_path, tiny_png()).unwrap();

    service
        .write_image(WriteImageRequest {
            path: Some(image_path),
            bytes: None,
            prefer_raw_png: Some(true),
            also_set_standard_image: Some(true),
        })
        .unwrap();

    let image = service.read_image(None, dir.path()).unwrap();
    assert_eq!(image.width, 1);
}

#[test]
#[ignore = "Requires real OS clipboard"]
fn scenario_files_and_video_files() {
    let service = ClipboardService::new_native().unwrap();
    let dir = tempdir().unwrap();

    let image_file = dir.path().join("a.png");
    let video_file = dir.path().join("b.mp4");
    let doc_file = dir.path().join("c.txt");

    fs::write(&image_file, tiny_png()).unwrap();
    fs::write(&video_file, b"fake-video-bytes").unwrap();
    fs::write(&doc_file, b"hello").unwrap();

    service
        .write_files(vec![
            image_file.to_string_lossy().to_string(),
            video_file.to_string_lossy().to_string(),
            doc_file.to_string_lossy().to_string(),
        ])
        .unwrap();

    let files = service.read_files().unwrap();
    assert!(files.files.len() >= 3);

    service
        .write_video_files(vec![video_file.to_string_lossy().to_string()])
        .unwrap();
}

#[test]
#[ignore = "Requires real OS clipboard"]
fn scenario_buffer_roundtrip() {
    let service = ClipboardService::new_native().unwrap();
    service
        .write_buffer(BufferPayload {
            format: "application/x-demo-binary".to_string(),
            data: vec![1, 2, 3, 4, 5],
        })
        .unwrap();

    let payload = service.read_buffer("application/x-demo-binary").unwrap();
    assert_eq!(payload.data, vec![1, 2, 3, 4, 5]);
}

#[test]
#[ignore = "Requires real OS clipboard"]
fn scenario_snapshot_and_clear() {
    let service = ClipboardService::new_native().unwrap();
    service.write_text("snapshot text".to_string()).unwrap();

    let dir = tempdir().unwrap();
    let snapshot = service.read_clipboard(None, dir.path()).unwrap();
    assert!(snapshot.available_formats.len() >= 1);

    service.clear().unwrap();
}

#[test]
#[ignore = "Requires real OS clipboard"]
fn scenario_available_formats_and_has_format() {
    let service = ClipboardService::new_native().unwrap();
    service.write_text("fmt".to_string()).unwrap();
    let formats = service.available_formats().unwrap();
    assert!(!formats.is_empty());
    assert!(service.has_format("text/plain").is_ok());
}
