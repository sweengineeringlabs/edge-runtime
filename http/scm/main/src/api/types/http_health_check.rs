//! HTTP health check result type.

/// Health check result returned by [`HttpIngress::health_check`].
///
/// [`HttpIngress::health_check`]: crate::api::traits::HttpIngress::health_check
#[derive(Debug, Clone)]
pub struct HttpHealthCheck {
    /// `true` when the handler is healthy.
    pub healthy: bool,
    /// Optional human-readable status detail.
    pub message: Option<String>,
}

impl HttpHealthCheck {
    /// Construct a healthy result.
    pub fn healthy() -> Self {
        Self {
            healthy: true,
            message: None,
        }
    }

    /// Construct an unhealthy result with a status message.
    pub fn unhealthy(msg: impl Into<String>) -> Self {
        Self {
            healthy: false,
            message: Some(msg.into()),
        }
    }
}
