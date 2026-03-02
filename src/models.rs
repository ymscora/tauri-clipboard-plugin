use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ReadImageOptions {
    pub include_bytes: Option<bool>,
    pub save_to: Option<PathBuf>,
    pub auto_save: Option<bool>,
    pub prefer_raw_png: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ReadImage {
    pub path: Option<PathBuf>,
    pub width: u32,
    pub height: u32,
    pub size: u64,
    pub format: String,
    pub bytes: Option<Vec<u8>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum FileKind {
    Image,
    Video,
    Directory,
    File,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct FileItem {
    pub path: String,
    pub size: u64,
    pub is_dir: bool,
    pub kind: FileKind,
    pub mime: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ReadFiles {
    pub files: Vec<FileItem>,
    pub size: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct BufferPayload {
    pub format: String,
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct WriteImageRequest {
    pub path: Option<PathBuf>,
    pub bytes: Option<Vec<u8>>,
    pub prefer_raw_png: Option<bool>,
    pub also_set_standard_image: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct WriteImageBytesRequest {
    pub bytes: Vec<u8>,
    pub format: Option<String>,
    pub fast_only: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ClipboardSnapshot {
    pub available_formats: Vec<String>,
    pub text: Option<String>,
    pub rtf: Option<String>,
    pub html: Option<String>,
    pub image: Option<ReadImage>,
    pub files: Option<ReadFiles>,
}
