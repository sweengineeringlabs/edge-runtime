//! Domain error type for swe_edge_runtime_isolator.

/// Domain error type for the swe_edge_runtime_isolator crate.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// An I/O error occurred.
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    /// An operation failed.
    #[error("Operation failed: {message}")]
    Operation { message: String },
}
