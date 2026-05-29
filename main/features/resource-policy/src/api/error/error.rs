//! Domain service error type for swe_edge_runtime_resource_policy.

/// Errors that can occur in swe_edge_runtime_resource_policy service operations.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// An I/O error occurred.
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    /// An operation failed.
    #[error("Operation failed: {message}")]
    Operation { message: String },
}
