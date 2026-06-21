//! Observer trait for reading gRPC server state without mutating it.

use std::sync::Arc;

use swe_edge_ingress_grpc::HealthService;

/// Read-only observer over a configured [`crate::TonicGrpcServer`].
pub trait GrpcServerObserver: Send + Sync {
    /// Returns `true` if gRPC reflection is enabled.
    fn is_reflection_enabled(&self) -> bool;

    /// Returns the auto-wired health service, or `None` if disabled.
    fn health_service(&self) -> Option<&Arc<HealthService>>;
}
