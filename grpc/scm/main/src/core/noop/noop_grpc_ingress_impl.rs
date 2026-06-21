//! No-op gRPC ingress implementation.

use std::sync::Arc;

use futures::future::BoxFuture;
use swe_edge_ingress_grpc::{
    GrpcHealthCheck, GrpcIngress, GrpcIngressResult, GrpcMetadata, GrpcRequest, GrpcResponse,
    SecurityContext,
};

/// No-op gRPC ingress — used in tests and as a placeholder.
pub struct NoopGrpcIngress;

impl GrpcIngress for NoopGrpcIngress {
    fn handle_unary(
        &self,
        _req: GrpcRequest,
        _ctx: SecurityContext,
    ) -> BoxFuture<'_, GrpcIngressResult<GrpcResponse>> {
        Box::pin(async {
            Ok(GrpcResponse {
                body: vec![],
                metadata: GrpcMetadata::default(),
            })
        })
    }

    fn health_check(&self) -> BoxFuture<'_, GrpcIngressResult<GrpcHealthCheck>> {
        Box::pin(async { Ok(GrpcHealthCheck::healthy()) })
    }
}

impl NoopGrpcIngress {
    /// Create a reference-counted no-op ingress.
    pub fn create() -> Arc<Self> {
        Arc::new(Self)
    }
}
