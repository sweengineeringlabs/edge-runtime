//! Error type for HTTP inbound operations.

/// Error type for HTTP ingress operations.
#[derive(Debug, thiserror::Error)]
pub enum HttpIngressError {
    /// Internal server error.
    #[error("internal: {0}")]
    Internal(String),
    /// Requested resource not found.
    #[error("not found: {0}")]
    NotFound(String),
    /// Request input failed validation.
    #[error("invalid input: {0}")]
    InvalidInput(String),
    /// Upstream service unavailable.
    #[error("unavailable: {0}")]
    Unavailable(String),
    /// Operation timed out.
    #[error("timeout: {0}")]
    Timeout(String),
    /// Caller is not authenticated.
    #[error("unauthorized: {0}")]
    Unauthorized(String),
    /// Caller lacks the required permission.
    #[error("permission denied: {0}")]
    PermissionDenied(String),
    /// Operation conflicts with existing state.
    #[error("conflict: {0}")]
    Conflict(String),
    /// Handler does not support the requested method (HTTP 405).
    #[error("method not allowed: {0}")]
    MethodNotAllowed(String),
    /// Valid request rejected by a business rule (HTTP 422).
    #[error("unprocessable entity: {0}")]
    UnprocessableEntity(String),
}
