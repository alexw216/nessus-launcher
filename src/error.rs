//! Error types for the `nessus-launcher` library.

use std::fmt;
use std::io;

/// A convenient result type used throughout the library.
pub type Result<T> = std::result::Result<T, NessusError>;

/// The error type for all operations in `nessus-launcher`.
#[derive(Debug)]
pub enum NessusError {
    /// Errors originating from HTTP requests or responses.
    Http(reqwest::Error),

    /// Errors originating from JSON parsing or serialization.
    Json(serde_json::Error),

    /// Errors related to environment variables or configuration.
    Config(String),

    /// I/O related errors.
    Io(io::Error),

    /// A generic error with a human-readable message.
    Other(String),
}

impl fmt::Display for NessusError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NessusError::Http(e) => write!(f, "HTTP error: {e}"),
            NessusError::Json(e) => write!(f, "JSON error: {e}"),
            NessusError::Config(msg) => write!(f, "Configuration error: {msg}"),
            NessusError::Io(e) => write!(f, "I/O error: {e}"),
            NessusError::Other(msg) => write!(f, "Error: {msg}"),
        }
    }
}

impl std::error::Error for NessusError {}

impl From<reqwest::Error> for NessusError {
    fn from(e: reqwest::Error) -> Self {
        NessusError::Http(e)
    }
}

impl From<serde_json::Error> for NessusError {
    fn from(e: serde_json::Error) -> Self {
        NessusError::Json(e)
    }
}

impl From<io::Error> for NessusError {
    fn from(e: io::Error) -> Self {
        NessusError::Io(e)
    }
}

