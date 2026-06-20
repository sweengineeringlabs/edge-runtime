//! No-op implementation of [`crate::api::GrpcIngress`].

use edge_domain::SecurityContext;
use futures::future::BoxFuture;

use crate::api::{GrpcHealthCheck, GrpcIngress, GrpcIngressResult, GrpcRequest, GrpcResponse};

impl GrpcIngress for crate::api::NoopGrpcIngress {
    fn handle_unary(
        &self,
        _req: &GrpcRequest,
        _ctx: SecurityContext,
    ) -> BoxFuture<'_, GrpcIngressResult<GrpcResponse>> {
        Box::pin(async { Ok(GrpcResponse::empty()) })
    }

    fn health_check(&self) -> BoxFuture<'_, GrpcIngressResult<GrpcHealthCheck>> {
        Box::pin(async { Ok(GrpcHealthCheck::healthy()) })
    }
}
