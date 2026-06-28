//! NoopGrpcValidator type declaration and inherent impl.

use std::sync::Arc;

/// No-op validator — always passes.
pub struct NoopGrpcValidator;

impl NoopGrpcValidator {
    /// Wrap a new `NoopGrpcValidator` in an `Arc` for use as a [`crate::api::Validator`] trait object.
    pub fn create() -> Arc<Self> {
        Arc::new(Self)
    }
}
