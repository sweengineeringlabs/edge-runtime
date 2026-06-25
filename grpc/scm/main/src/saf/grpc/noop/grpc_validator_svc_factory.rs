//! SAF factory surface — NoopGrpcValidator inherent methods.

use std::sync::Arc;

use crate::api::NoopGrpcValidator;

impl NoopGrpcValidator {
    /// Wrap a new `NoopGrpcValidator` in an `Arc` for use as a [`Validator`] trait object.
    pub fn create() -> Arc<Self> {
        Arc::new(Self)
    }
}
