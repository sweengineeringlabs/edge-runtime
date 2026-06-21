//! Integration tests for NoopGrpcIngress.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::time::Duration;

use swe_edge_runtime_grpc::{
    GrpcIngress, GrpcRequest, NoopGrpcIngress, SecurityContext,
};

#[tokio::test]
async fn test_noop_grpc_ingress_handle_unary_returns_empty_body() {
    let ingress = NoopGrpcIngress;
    let req = GrpcRequest::new("/pkg.Svc/Method".to_string(), vec![1, 2, 3], Duration::from_secs(5));
    let ctx = SecurityContext::unauthenticated();
    let resp = ingress.handle_unary(req, ctx).await.unwrap();
    assert!(resp.body.is_empty());
}

#[tokio::test]
async fn test_noop_grpc_ingress_health_check_returns_healthy() {
    let ingress = NoopGrpcIngress;
    let result = ingress.health_check().await.unwrap();
    assert!(result.healthy);
}

#[test]
fn test_noop_grpc_ingress_create_returns_arc() {
    let _ = NoopGrpcIngress::create();
}
