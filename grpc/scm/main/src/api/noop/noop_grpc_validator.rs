//! NoopGrpcValidator type declaration — SEA api/ is the legal declaration site.

use std::sync::Arc;

/// No-op validator — always passes.
pub struct NoopGrpcValidator;

impl NoopGrpcValidator {
    /// Create a reference-counted no-op validator.
    pub fn create() -> Arc<Self> {
        Arc::new(Self)
    }
}
