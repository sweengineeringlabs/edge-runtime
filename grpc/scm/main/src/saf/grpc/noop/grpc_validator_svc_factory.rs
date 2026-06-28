//! SAF surface for NoopGrpcValidator — no-op validator factory.
use std::sync::Arc;

use crate::api::NoopGrpcValidator;

/// Service identifier for the no-op gRPC validator.
pub const NOOP_GRPC_VALIDATOR_SVC: &str = "noop_grpc_validator";

impl NoopGrpcValidator {
    /// Wrap a new `NoopGrpcValidator` in an `Arc` for use as a [`crate::api::Validator`] trait object.
    pub fn create() -> Arc<Self> {
        Arc::new(Self)
    }
}
