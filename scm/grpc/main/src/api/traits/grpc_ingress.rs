//! gRPC inbound contract — receives and handles inbound gRPC calls.

use edge_domain::SecurityContext;
use futures::future::BoxFuture;

use crate::api::{GrpcHealthCheck, GrpcIngressError, GrpcIngressResult, GrpcRequest, GrpcResponse};

/// Receives and handles inbound gRPC calls.
///
/// Implement this trait in a plugin or transport binding. The composition root
/// wires implementors into the gRPC transport crate which drives the server loop.
pub trait GrpcIngress: Send + Sync {
    /// Handle a unary gRPC call and return a response.
    fn handle_unary(
        &self,
        req: &GrpcRequest,
        ctx: SecurityContext,
    ) -> BoxFuture<'_, GrpcIngressResult<GrpcResponse>>;

    /// Perform a health check of this ingress handler.
    fn health_check(&self) -> BoxFuture<'_, GrpcIngressResult<GrpcHealthCheck>>;

    /// Return the error kind label for a given ingress error.
    fn error_kind(&self, _err: &GrpcIngressError) -> &'static str {
        "grpc_ingress_error"
    }
}
