//! NoopGrpcIngress type declaration and inherent impl.

use std::sync::Arc;

/// No-op gRPC ingress — used in tests and as a placeholder.
pub struct NoopGrpcIngress;

impl NoopGrpcIngress {
    /// Wrap a new `NoopGrpcIngress` in an `Arc` for use as a [`crate::api::GrpcIngress`] trait object.
    pub fn create() -> Arc<Self> {
        Arc::new(Self)
    }
}
