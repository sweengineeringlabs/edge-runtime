//! Error type for gRPC ingress operations.

/// Error returned by [`crate::GrpcIngress`] implementations.
#[derive(Debug, thiserror::Error)]
pub enum GrpcIngressError {
    /// An unexpected internal error occurred.
    #[error("internal error: {0}")]
    Internal(String),
    /// The requested RPC method was not found.
    #[error("not found: {0}")]
    NotFound(String),
    /// The request was malformed or failed validation.
    #[error("invalid input: {0}")]
    InvalidInput(String),
    /// The service is temporarily unavailable.
    #[error("unavailable: {0}")]
    Unavailable(String),
    /// The operation timed out.
    #[error("timeout: {0}")]
    Timeout(String),
    /// The caller is not authenticated.
    #[error("unauthorized: {0}")]
    Unauthorized(String),
    /// The caller is authenticated but lacks permission.
    #[error("permission denied: {0}")]
    PermissionDenied(String),
    /// The RPC method is not implemented by this handler.
    #[error("unimplemented: {0}")]
    Unimplemented(String),
}
