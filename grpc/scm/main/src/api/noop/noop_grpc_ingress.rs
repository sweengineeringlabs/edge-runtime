//! NoopGrpcIngress type declaration — SEA api/ is the legal declaration site.

use std::sync::Arc;

/// No-op gRPC ingress — used in tests and as a placeholder.
pub struct NoopGrpcIngress;

impl NoopGrpcIngress {
    /// Create a reference-counted no-op ingress.
    pub fn create() -> Arc<Self> {
        Arc::new(Self)
    }
}
