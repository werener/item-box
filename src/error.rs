// src/error.rs
use thiserror::Error;

#[derive(Debug, Error)]
pub enum StorageError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Web storage error: {0}")]
    WebStorage(String),
}

#[derive(Debug, Error)]
pub enum ItemError {
    #[error("Name has to be non-empty")]
    InvalidName,

    #[error("Price has to be 0 or higher")]
    InvalidPrice,
}
