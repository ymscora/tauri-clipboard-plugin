use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ClipboardError {
    #[error("invalid argument: {0}")]
    InvalidArgument(String),

    #[error("clipboard backend error: {0}")]
    Backend(String),

    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("resource not found: {0}")]
    NotFound(String),

    #[error("unsupported operation: {0}")]
    Unsupported(String),
}

impl ClipboardError {
    pub fn code(&self) -> &'static str {
        match self {
            Self::InvalidArgument(_) => "INVALID_ARGUMENT",
            Self::Backend(_) => "CLIPBOARD_BACKEND_ERROR",
            Self::Io(_) => "IO_ERROR",
            Self::NotFound(_) => "NOT_FOUND",
            Self::Unsupported(_) => "UNSUPPORTED",
        }
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiError {
    pub code: &'static str,
    pub message: String,
}

impl From<ClipboardError> for ApiError {
    fn from(value: ClipboardError) -> Self {
        Self {
            code: value.code(),
            message: value.to_string(),
        }
    }
}

pub type Result<T> = std::result::Result<T, ClipboardError>;
pub type ApiResult<T> = std::result::Result<T, ApiError>;
