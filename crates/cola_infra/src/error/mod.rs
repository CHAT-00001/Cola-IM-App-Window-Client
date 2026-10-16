// src/error/mod.rs -- Cola SDK error model
// 2026/10/9 00:20 Created.

////////

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum SdkError {
    InvalidContext(String),
    InvalidBatch(String),
    CursorMismatch,
    Storage(String),
    NotFound(String),
}

pub type Result<T> = std::result::Result<T, SdkError>;

//////// END
