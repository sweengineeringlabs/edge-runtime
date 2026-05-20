//! `RuntimeError` — errors from runtime manager operations.

use thiserror::Error;

/// Errors that can occur during runtime manager operations.
#[derive(Debug, Error)]
pub enum RuntimeError {
    /// Runtime failed to start one or more components.
    #[error("start failed: {0}")]
    StartFailed(String),
    /// Runtime failed to cleanly stop one or more components.
    #[error("shutdown failed: {0}")]
    ShutdownFailed(String),
    /// Could not bind the server socket to the configured address.
    #[error("bind failed: {0}")]
    BindFailed(String),
    /// Unexpected internal error.
    #[error("internal: {0}")]
    Internal(String),
    /// Graceful shutdown exceeded the configured timeout (seconds).
    #[error("shutdown timed out after {0}s")]
    ShutdownTimeout(u64),
}

impl From<crate::api::config::ConfigError> for RuntimeError {
    fn from(e: crate::api::config::ConfigError) -> Self {
        RuntimeError::StartFailed(e.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_runtime_error_display_start_failed() {
        let e = RuntimeError::StartFailed("port in use".into());
        assert_eq!(e.to_string(), "start failed: port in use");
    }

    #[test]
    fn test_runtime_error_display_shutdown_failed() {
        let e = RuntimeError::ShutdownFailed("timeout".into());
        assert_eq!(e.to_string(), "shutdown failed: timeout");
    }

    #[test]
    fn test_runtime_error_display_bind_failed() {
        let e = RuntimeError::BindFailed("addr already in use".into());
        assert_eq!(e.to_string(), "bind failed: addr already in use");
    }

    #[test]
    fn test_runtime_error_display_shutdown_timeout() {
        let e = RuntimeError::ShutdownTimeout(30);
        assert_eq!(e.to_string(), "shutdown timed out after 30s");
    }
}
