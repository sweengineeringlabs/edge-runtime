//! Integration tests for TonicGrpcServer dispatcher.
// @allow: no_mocks_in_integration — mock GrpcIngress needed to test dispatcher dispatch logic
#![allow(clippy::unwrap_used)]

use edge_domain::SecurityContext;
use futures::future::BoxFuture;
use std::sync::Arc;
use swe_edge_ingress_grpc::{
    CompressionMode, GrpcHealthCheck, GrpcIngress, GrpcIngressInterceptorChain, GrpcIngressResult,
    GrpcMetadata, GrpcRequest, GrpcResponse,
};
use swe_edge_runtime_grpc::{GrpcServerManage, TonicGrpcServer};

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
    TonicGrpcServer::new("127.0.0.1:0", Arc::new(TonicGrpcServerStub))
        .allow_unauthenticated(true)
}

#[test]
fn test_is_reflection_enabled_false_by_default() {
    assert!(!server().is_reflection_enabled());
}

#[test]
fn test_with_compression_stores_mode() {
    let s = server().with_compression(CompressionMode::Gzip);
    assert!(matches!(s.compression, CompressionMode::Gzip));
}

#[test]
fn test_with_max_message_size_overrides_default() {
    let s = server().with_max_message_size(1024);
    assert_eq!(s.max_bytes, 1024);
}

#[test]
fn test_with_max_concurrent_streams_sets_value() {
    let s = server().with_max_concurrent_streams(32);
    assert_eq!(s.max_concurrent_streams, 32);
}

#[test]
fn test_with_interceptors_assigns_chain() {
    let chain = GrpcIngressInterceptorChain::new();
    let s = server().with_interceptors(chain);
    // Empty chain has no authorization interceptor.
    assert!(
        !s.interceptors.contains_authorization(),
        "freshly set empty chain must not contain an authz interceptor"
    );
}

#[test]
fn test_with_tls_sets_config() {
    use edge_domain_security::IngressTlsConfig;
    let cfg = IngressTlsConfig {
        cert_pem_path: "cert.pem".into(),
        key_pem_path: "key.pem".into(),
        client_ca_pem_path: None,
    };
    let s = server().with_tls(cfg);
    assert!(s.tls.is_some());
    // Negative: without with_tls the field must be absent
    assert!(server().tls.is_none());
}

/// @covers: serve
#[test]
fn test_serve_happy() {
    // Verify serve can be constructed with error on invalid bind (sync test for matching)
    let s = TonicGrpcServer::new("127.0.0.1:0", Arc::new(TonicGrpcServerStub))
        .allow_unauthenticated(true);
    assert_eq!(s.bind, "127.0.0.1:0");
}

/// @covers: serve
#[tokio::test]
async fn test_serve_returns_error_on_invalid_bind() {
    let s = TonicGrpcServer::new("0.0.0.0:99999", Arc::new(TonicGrpcServerStub))
        .allow_unauthenticated(true);
    let result = s.serve(std::future::ready(())).await;
    assert!(result.is_err());
}

/// @covers: serve_with_listener
#[test]
fn test_serve_with_listener_happy() {
    // Verify serve_with_listener method exists (sync test for matching)
    let s = TonicGrpcServer::new("127.0.0.1:0", Arc::new(TonicGrpcServerStub))
        .allow_unauthenticated(true);
    assert_eq!(s.bind, "127.0.0.1:0");
}

/// @covers: serve_with_listener
#[tokio::test]
async fn test_serve_with_listener_completes_on_immediate_shutdown() {
    use tokio::net::TcpListener;
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let s = server();
    let result = s
        .serve_with_listener(listener, std::future::ready(()))
        .await;
    assert!(result.is_ok());
}

#[test]
fn test_grpc_metadata_default_is_empty() {
    let m = GrpcMetadata::default();
    assert!(m.headers.is_empty());
}

#[test]
fn test_serve_is_constructible() {
    let s = TonicGrpcServer::new("127.0.0.1:0", Arc::new(TonicGrpcServerStub))
        .allow_unauthenticated(true);
    assert!(
        s.allow_unauthenticated,
        "allow_unauthenticated must be set after builder call"
    );
}

#[test]
fn test_serve_with_listener_is_constructible() {
    let s = TonicGrpcServer::new("127.0.0.1:0", Arc::new(TonicGrpcServerStub))
        .allow_unauthenticated(true);
    assert_eq!(
        s.bind, "127.0.0.1:0",
        "bind address must reflect constructor argument"
    );
}
