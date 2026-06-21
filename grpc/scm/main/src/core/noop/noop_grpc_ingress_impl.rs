//! No-op gRPC ingress implementation.

use futures::future::BoxFuture;
use swe_edge_ingress_grpc::{
    GrpcHealthCheck, GrpcIngress, GrpcIngressResult, GrpcMetadata, GrpcRequest, GrpcResponse,
    SecurityContext,
};

use crate::api::NoopGrpcIngress;

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
