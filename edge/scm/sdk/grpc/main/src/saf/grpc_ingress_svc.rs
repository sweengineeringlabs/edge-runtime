//! SAF surface for [`crate::api::GrpcIngress`] and [`crate::api::types::NoopGrpcIngress`].

pub use crate::api::{
    GrpcHealthCheck, GrpcIngress, GrpcIngressError, GrpcIngressResult, GrpcMethod, GrpcRequest,
    GrpcResponse, NoopGrpcIngress,
};

impl NoopGrpcIngress {
    /// Create a new [`NoopGrpcIngress`] — a pass-through ingress handler for tests and
    /// composition roots that have not yet wired a real ingress implementation.
    pub fn create() -> Self {
        Self
    }
}
