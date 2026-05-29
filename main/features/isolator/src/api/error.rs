//! Error types for swe_edge_runtime_isolator.

/// Errors that can occur in swe_edge_runtime_isolator.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// An I/O error occurred.
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    /// An operation failed.
    #[error("Operation failed: {message}")]
    Operation { message: String },
}
