//! Integration tests for TonicGrpcServer dispatcher.
// @allow: no_mocks_in_integration — mock GrpcIngress needed to test dispatcher dispatch logic
#![allow(clippy::unwrap_used)]

use edge_domain::SecurityContext;
use futures::future::BoxFuture;
use std::sync::Arc;
use swe_edge_ingress_grpc::{
    AuthorizationInterceptor, GrpcHealthCheck, GrpcIngress, GrpcIngressError,
    GrpcIngressInterceptor, GrpcIngressInterceptorChain, GrpcIngressResult, GrpcMetadata,
    GrpcRequest, GrpcResponse,
};
use swe_edge_runtime_grpc::{GrpcServerManage, TonicGrpcServer};

struct FakeAuthzInterceptor;
impl GrpcIngressInterceptor for FakeAuthzInterceptor {
    fn before_dispatch(&self, _: &mut GrpcRequest) -> Result<(), GrpcIngressError> {
        Ok(())
    }
    fn after_dispatch(&self, _: &mut GrpcResponse) -> Result<(), GrpcIngressError> {
        Ok(())
    }
    fn is_authorization(&self) -> bool {
        true
    }
}
impl AuthorizationInterceptor for FakeAuthzInterceptor {}

struct TonicGrpcServerStub;
impl GrpcIngress for TonicGrpcServerStub {
    fn handle_unary(
        &self,
        _: GrpcRequest,
        _ctx: SecurityContext,
    ) -> BoxFuture<'_, GrpcIngressResult<GrpcResponse>> {
        Box::pin(async {
            Ok(GrpcResponse {
                body: vec![],
                metadata: Default::default(),
            })
        })
    }
    fn health_check(&self) -> BoxFuture<'_, GrpcIngressResult<GrpcHealthCheck>> {
        Box::pin(async {
            Ok(GrpcHealthCheck {
                healthy: true,
                message: None,
            })
        })
    }
}

fn server() -> TonicGrpcServer {
    TonicGrpcServer::new("127.0.0.1:0", Arc::new(TonicGrpcServerStub)).allow_unauthenticated(true)
}

#[test]
fn test_is_reflection_enabled_false_by_default() {
    assert!(!server().is_reflection_enabled());
}

#[test]
fn test_with_max_message_size_overrides_default() {
    let s = server().with_max_message_size(1024);
    assert_eq!(s.max_message_size(), 1024);
}

#[test]
fn test_with_max_concurrent_streams_sets_value() {
    let s = server().with_max_concurrent_streams(32);
    assert_eq!(s.max_concurrent_streams(), 32);
}

/// @covers: serve
#[tokio::test]
async fn test_serve_immediate_shutdown_happy() {
    // Binds 127.0.0.1:0 then returns Ok once the already-ready shutdown
    // future fires — exercises the bind + serve_with_listener path of serve.
    let s = server();
    let result = s.serve(std::future::ready(())).await;
    assert!(
        result.is_ok(),
        "serve must return Ok when shutdown fires before any connection: {result:?}"
    );
}

/// @covers: serve
#[tokio::test]
async fn test_serve_invalid_bind_error() {
    // Port 99999 is out of the valid u16 range → bind fails → serve errors.
    let s = TonicGrpcServer::new("0.0.0.0:99999", Arc::new(TonicGrpcServerStub))
        .allow_unauthenticated(true);
    let result = s.serve(std::future::ready(())).await;
    assert!(result.is_err(), "out-of-range port must fail to bind");
}

/// @covers: serve_with_listener
#[tokio::test]
async fn test_serve_with_listener_immediate_shutdown_happy() {
    use tokio::net::TcpListener;
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let s = server();
    let result = s
        .serve_with_listener(listener, std::future::ready(()))
        .await;
    assert!(
        result.is_ok(),
        "serve_with_listener must return Ok on immediate shutdown: {result:?}"
    );
}

/// @covers: serve_with_listener
#[tokio::test]
async fn test_serve_with_listener_with_authz_serves_happy() {
    use tokio::net::TcpListener;
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    // A registered authorization interceptor satisfies the fail-closed invariant
    // even without allow_unauthenticated, so the server serves and exits cleanly.
    let chain = GrpcIngressInterceptorChain::new().push(Arc::new(FakeAuthzInterceptor));
    let s =
        TonicGrpcServer::new("127.0.0.1:0", Arc::new(TonicGrpcServerStub)).with_interceptors(chain);
    let result = s
        .serve_with_listener(listener, std::future::ready(()))
        .await;
    assert!(
        result.is_ok(),
        "an authz interceptor must satisfy the fail-closed invariant and serve: {result:?}"
    );
}

/// @covers: serve_with_listener
#[tokio::test]
async fn test_serve_with_listener_missing_authz_error() {
    use tokio::net::TcpListener;
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    // No authorization interceptor and allow_unauthenticated defaults to false,
    // so the fail-closed invariant must reject before the serve loop starts.
    let s = TonicGrpcServer::new("127.0.0.1:0", Arc::new(TonicGrpcServerStub));
    let result = s
        .serve_with_listener(listener, std::future::ready(()))
        .await;
    assert!(
        result.is_err(),
        "serve_with_listener must fail closed when no authz interceptor is registered"
    );
}

#[test]
fn test_grpc_metadata_default_is_empty() {
    let m = GrpcMetadata::default();
    assert!(m.headers.is_empty());
}
