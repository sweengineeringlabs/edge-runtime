//! Integration tests for DefaultIngress.

use std::sync::Arc;
use swe_edge_runtime::{DefaultIngress, Ingress};

/// @covers: DefaultIngress
#[test]
fn test_default_ingress_empty_has_no_transports() {
    let i = DefaultIngress::empty();
    assert!(!i.has_any());
}

/// @covers: DefaultIngress
#[test]
fn test_default_ingress_empty_http_returns_none() {
    let i = DefaultIngress::empty();
    assert!(i.http().is_none());
}

/// @covers: DefaultIngress
#[test]
fn test_default_ingress_empty_grpc_returns_none() {
    let i = DefaultIngress::empty();
    assert!(i.grpc().is_none());
}

/// @covers: DefaultIngress
#[test]
fn test_default_ingress_with_grpc_has_any_returns_true() {
    use edge_domain::RequestContext;
    use futures::future::BoxFuture;
    use swe_edge_ingress_grpc::{
        GrpcHealthCheck, GrpcIngressError, GrpcIngressResult, GrpcMessageStream, GrpcMetadata,
        GrpcRequest, GrpcResponse,
    };
    struct Stub;
    impl swe_edge_runtime::GrpcIngress for Stub {
        fn handle_unary(
            &self,
            _: GrpcRequest,
            _: RequestContext,
        ) -> BoxFuture<'_, GrpcIngressResult<GrpcResponse>> {
            Box::pin(async { Err(GrpcIngressError::Unimplemented("stub".into())) })
        }
        fn handle_stream(
            &self,
            _: String,
            _: GrpcMetadata,
            _: GrpcMessageStream,
            _: RequestContext,
        ) -> BoxFuture<'_, GrpcIngressResult<(GrpcMessageStream, GrpcMetadata)>> {
            Box::pin(async { Err(GrpcIngressError::Unimplemented("stub".into())) })
        }
        fn health_check(&self) -> BoxFuture<'_, GrpcIngressResult<GrpcHealthCheck>> {
            Box::pin(async { Ok(GrpcHealthCheck::healthy()) })
        }
    }
    let i = DefaultIngress::new_grpc(Arc::new(Stub));
    assert!(i.has_any());
}
