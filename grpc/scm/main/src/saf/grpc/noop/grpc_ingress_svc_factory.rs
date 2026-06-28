//! SAF surface for NoopGrpcIngress — no-op gRPC ingress factory.
use std::sync::Arc;

use crate::api::NoopGrpcIngress;

/// Service identifier for the no-op gRPC ingress.
pub const NOOP_GRPC_INGRESS_SVC: &str = "noop_grpc_ingress";

impl NoopGrpcIngress {
    /// Wrap a new `NoopGrpcIngress` in an `Arc` for use as a [`crate::api::GrpcIngress`] trait object.
    pub fn create() -> Arc<Self> {
        Arc::new(Self)
    }
}
