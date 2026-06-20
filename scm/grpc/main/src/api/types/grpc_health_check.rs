//! gRPC ingress health-check result.

/// Result of a [`crate::GrpcIngress::health_check`] call.
#[derive(Debug, Clone)]
pub struct GrpcHealthCheck {
    /// `true` when the handler is healthy and ready to serve requests.
    pub healthy: bool,
    /// Optional human-readable status detail.
    pub message: Option<String>,
}

impl GrpcHealthCheck {
    /// Construct a healthy result with no detail message.
    pub fn healthy() -> Self {
        Self {
            healthy: true,
            message: None,
        }
    }

    /// Construct an unhealthy result with a detail message.
    pub fn unhealthy(message: impl Into<String>) -> Self {
        Self {
            healthy: false,
            message: Some(message.into()),
        }
    }
}
