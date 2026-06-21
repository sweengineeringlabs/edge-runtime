//! gRPC server — GrpcServerObserver and GrpcServer trait implementations.

use std::sync::Arc;

use futures::future::BoxFuture;
use swe_edge_ingress_grpc::HealthService;

use crate::api::{GrpcServer, GrpcServerError, GrpcServerObserver, TonicGrpcServer};

impl GrpcServerObserver for TonicGrpcServer {
    fn is_reflection_enabled(&self) -> bool {
        self.enable_reflection
    }

    fn health_service(&self) -> Option<&Arc<HealthService>> {
        self.health_service.as_ref()
    }
}

impl GrpcServer for TonicGrpcServer {
    fn serve(
        &self,
        shutdown: BoxFuture<'static, ()>,
    ) -> BoxFuture<'_, Result<(), GrpcServerError>> {
        // Delegate to the inherent async serve method on TonicGrpcServer,
        // which accepts any Future<Output=()>. Boxing here satisfies the trait.
        Box::pin(TonicGrpcServer::serve(self, shutdown))
    }
}
