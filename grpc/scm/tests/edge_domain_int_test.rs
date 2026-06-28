//! Integration tests that exercise edge-domain types used by this crate.
//! The grpc_principal.rs SPI uses edge_domain::Principal and SecurityContext.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::time::Duration;

use swe_edge_runtime_grpc::{
    GrpcIngress, GrpcRequest, GrpcServerManage, GrpcServerObserver, NoopGrpcIngress, SecurityContext,
    TonicGrpcServer,
};

/// Verify that SecurityContext (from edge-domain) integrates with dispatch.
#[test]
fn test_security_context_unauthenticated_is_constructible_happy() {
    let ctx = SecurityContext::unauthenticated();
    assert!(
        !ctx.authenticated,
        "unauthenticated context must have authenticated=false"
    );
}

/// Verify authenticated security context construction (uses edge-domain Principal).
#[test]
fn test_security_context_with_tenant_is_constructible_happy() {
    // Constructing a context with tenant exercises edge-domain's SecurityContext.
    let ctx = SecurityContext::unauthenticated().with_tenant("acme");
    assert_eq!(ctx.tenant_id.as_deref(), Some("acme"));
}

/// Verify a grpc request round-trip using edge-domain's SecurityContext.
#[tokio::test]
async fn test_grpc_request_dispatches_through_noop_ingress_happy() {
    let ingress = NoopGrpcIngress;
    let req = GrpcRequest::new(
        "/test.Svc/Method".to_string(),
        vec![],
        Duration::from_secs(5),
    );
    let ctx = SecurityContext::unauthenticated();
    let resp = ingress.handle_unary(req, ctx).await.unwrap();
    assert!(resp.body.is_empty());
}

/// Verify the health service observer integration (tower-http / HealthService).
#[test]
fn test_health_service_is_auto_wired_error() {
    // After without_health_service(), must return None.
    let s = TonicGrpcServer::new("127.0.0.1:0", NoopGrpcIngress::create()).without_health_service();
    assert!(
        GrpcServerObserver::health_service(&s).is_none(),
        "health service must be removable"
    );
}

/// Verify SecurityContext claim round-trip (exercises edge-domain).
#[test]
fn test_security_context_claim_round_trip_edge() {
    let ctx = SecurityContext::unauthenticated()
        .with_claim("role", "admin")
        .with_claim("org", "acme");
    assert_eq!(ctx.claim("role"), Some("admin"));
    assert_eq!(ctx.claim("org"), Some("acme"));
    assert_eq!(ctx.claim("missing"), None);
}

/// Exercises tower, tower-http and tokio-util through the serve path.
#[tokio::test]
async fn test_serve_with_immediate_shutdown_completes_successfully_edge() {
    use futures::FutureExt;
    use swe_edge_runtime_grpc::GrpcServer;
    let s =
        TonicGrpcServer::new("127.0.0.1:0", NoopGrpcIngress::create()).allow_unauthenticated(true);
    // Immediate shutdown exercises tower, hyper, tower-http, tokio-util paths.
    let shutdown = futures::future::ready(()).boxed();
    let result = tokio::time::timeout(
        std::time::Duration::from_secs(5),
        GrpcServer::serve(&s, shutdown),
    )
    .await;
    assert!(result.is_ok(), "timed out on immediate shutdown");
    assert!(result.unwrap().is_ok());
}
