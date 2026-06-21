//! gRPC server — GrpcServerObserver trait implementation.

use std::sync::Arc;

use swe_edge_ingress_grpc::HealthService;

use crate::api::server::traits::GrpcServerObserver;
use crate::api::server::types::TonicGrpcServer;

impl GrpcServerObserver for TonicGrpcServer {
    fn is_reflection_enabled(&self) -> bool {
        self.enable_reflection
    }

    fn health_service(&self) -> Option<&Arc<HealthService>> {
        self.health_service.as_ref()
    }
}
