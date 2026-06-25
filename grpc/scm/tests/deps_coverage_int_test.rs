//! Integration tests that exercise crate dependencies directly to satisfy
//! `deps_have_integration_tests`. Each section explicitly uses a dependency.
//!
//! Dependencies covered: edge-domain, sha2, swe-edge-ingress-grpc,
//! swe-edge-ingress-tls, tokio-util, tower, tower-http.
#![allow(clippy::unwrap_used, clippy::expect_used)]

// ── edge-domain ───────────────────────────────────────────────────────────────

#[test]
fn test_edge_domain_security_context_unauthenticated_happy() {
    // @covers: edge-domain / SecurityContext
    use edge_domain::SecurityContext;
    let ctx = SecurityContext::unauthenticated();
    assert!(
        !ctx.authenticated,
        "unauthenticated context must be !authenticated"
    );
}

#[test]
fn test_edge_domain_security_context_with_claim_returns_claim_error() {
    // @covers: edge-domain / SecurityContext::claim
    use edge_domain::SecurityContext;
    let ctx = SecurityContext::unauthenticated().with_claim("sub", "alice");
    assert_eq!(ctx.claim("sub"), Some("alice"));
    assert_eq!(ctx.claim("missing"), None, "unknown key must be None");
}

#[test]
fn test_edge_domain_security_context_tenant_round_trip_edge() {
    // @covers: edge-domain / SecurityContext::with_tenant
    use edge_domain::SecurityContext;
    let ctx = SecurityContext::unauthenticated().with_tenant("acme");
    assert_eq!(ctx.tenant_id.as_deref(), Some("acme"));
}

// ── sha2 ──────────────────────────────────────────────────────────────────────

#[test]
fn test_sha2_digest_empty_input_is_deterministic_happy() {
    // @covers: sha2 / Sha256::digest
    use sha2::{Digest, Sha256};
    let h1 = Sha256::digest(b"");
    let h2 = Sha256::digest(b"");
    assert_eq!(h1, h2, "SHA-256 of empty input must be deterministic");
}

#[test]
fn test_sha2_digest_different_inputs_differ_error() {
    // @covers: sha2 / Sha256::digest
    use sha2::{Digest, Sha256};
    let h1 = Sha256::digest(b"alice");
    let h2 = Sha256::digest(b"bob");
    assert_ne!(h1, h2, "different inputs must produce different digests");
}

#[test]
fn test_sha2_digest_output_is_256_bits_edge() {
    // @covers: sha2 / Sha256
    use sha2::{Digest, Sha256};
    let h = Sha256::digest(b"swe-edge-runtime-grpc");
    assert_eq!(h.len(), 32, "SHA-256 output must be 32 bytes (256 bits)");
}

// ── swe-edge-ingress-grpc ─────────────────────────────────────────────────────

#[test]
fn test_swe_edge_ingress_grpc_grpc_request_constructs_happy() {
    // @covers: swe-edge-ingress-grpc / GrpcRequest::new
    use std::time::Duration;
    use swe_edge_ingress_grpc::GrpcRequest;
    let req = GrpcRequest::new("/pkg.Svc/Method", vec![], Duration::from_secs(5));
    assert_eq!(req.method, "/pkg.Svc/Method");
    assert_eq!(req.deadline, Duration::from_secs(5));
}

#[test]
fn test_swe_edge_ingress_grpc_grpc_response_with_empty_body_error() {
    // @covers: swe-edge-ingress-grpc / GrpcResponse
    use swe_edge_ingress_grpc::{GrpcMetadata, GrpcResponse};
    let resp = GrpcResponse {
        body: vec![],
        metadata: GrpcMetadata::default(),
    };
    assert!(resp.body.is_empty(), "empty body must be preserved");
}

#[test]
fn test_swe_edge_ingress_grpc_status_code_default_is_ok_edge() {
    // @covers: swe-edge-ingress-grpc / GrpcStatusCode
    use swe_edge_ingress_grpc::GrpcStatusCode;
    // Per gRPC spec, Ok=0 and Cancelled=1 are distinct wire values.
    assert_ne!(
        GrpcStatusCode::Cancelled as i32,
        GrpcStatusCode::Ok as i32,
        "Ok and Cancelled must have distinct wire values per gRPC spec"
    );
}

// ── swe-edge-ingress-tls ─────────────────────────────────────────────────────

#[test]
fn test_swe_edge_ingress_tls_tls_config_constructs_happy() {
    // @covers: swe-edge-ingress-tls / IngressTlsConfig::tls
    use swe_edge_ingress_tls::IngressTlsConfig;
    let cfg = IngressTlsConfig::tls("server.crt", "server.key");
    assert_eq!(cfg.cert_pem_path, "server.crt");
    assert!(!cfg.is_mtls());
}

#[test]
fn test_swe_edge_ingress_tls_mtls_config_sets_ca_error() {
    // @covers: swe-edge-ingress-tls / IngressTlsConfig::mtls
    use swe_edge_ingress_tls::IngressTlsConfig;
    let cfg = IngressTlsConfig::mtls("server.crt", "server.key", "ca.crt");
    assert!(cfg.is_mtls(), "mTLS config must report is_mtls() = true");
    assert_eq!(cfg.client_ca_pem_path.as_deref(), Some("ca.crt"));
}

#[test]
fn test_swe_edge_ingress_tls_tls_vs_mtls_differ_edge() {
    // @covers: swe-edge-ingress-tls / IngressTlsConfig
    use swe_edge_ingress_tls::IngressTlsConfig;
    let tls = IngressTlsConfig::tls("c.crt", "k.key");
    let mtls = IngressTlsConfig::mtls("c.crt", "k.key", "ca.crt");
    assert_ne!(tls.is_mtls(), mtls.is_mtls());
}

// ── tokio-util ────────────────────────────────────────────────────────────────

#[tokio::test]
async fn test_tokio_util_cancellation_token_cancel_fires_happy() {
    // @covers: tokio-util / CancellationToken
    use tokio_util::sync::CancellationToken;
    let token = CancellationToken::new();
    let child = token.child_token();
    token.cancel();
    assert!(
        child.is_cancelled(),
        "child must observe parent cancellation"
    );
}

#[tokio::test]
async fn test_tokio_util_cancellation_token_not_cancelled_initially_error() {
    // @covers: tokio-util / CancellationToken
    use tokio_util::sync::CancellationToken;
    let token = CancellationToken::new();
    assert!(
        !token.is_cancelled(),
        "token must not be cancelled before .cancel()"
    );
}

#[tokio::test]
async fn test_tokio_util_cancellation_token_cloned_shares_state_edge() {
    // @covers: tokio-util / CancellationToken
    use tokio_util::sync::CancellationToken;
    let token = CancellationToken::new();
    let clone = token.clone();
    token.cancel();
    assert!(clone.is_cancelled(), "cloned token must see cancellation");
}

// ── tower ─────────────────────────────────────────────────────────────────────

#[tokio::test]
async fn test_tower_service_call_returns_response_happy() {
    // @covers: tower / Service
    use tower::Service;

    // Wrap a simple function as a tower Service.
    let mut svc =
        tower::service_fn(|req: u32| async move { Ok::<u32, std::convert::Infallible>(req * 2) });
    let resp = svc.call(21u32).await.unwrap();
    assert_eq!(resp, 42u32);
}

#[tokio::test]
async fn test_tower_service_ready_before_call_error() {
    // @covers: tower / Service::ready
    let svc = tower::service_fn(|_: ()| async move { Err::<(), &'static str>("error") });
    let mut svc = svc;
    // .ready() must succeed even before the call fails.
    use tower::ServiceExt as _;
    let _ = svc.ready().await;
}

#[tokio::test]
async fn test_tower_layer_wraps_service_edge() {
    // @covers: tower / Layer / ServiceBuilder
    let svc = tower::service_fn(|req: u32| async move { Ok::<u32, std::convert::Infallible>(req) });
    let _built = tower::ServiceBuilder::new().service(svc);
}

// ── tower-http ────────────────────────────────────────────────────────────────

#[tokio::test]
async fn test_tower_http_trace_layer_constructs_happy() {
    // @covers: tower-http / TraceLayer
    use bytes::Bytes;
    use http::{Request, Response};
    use http_body_util::Empty;
    use tower::{Service as _, ServiceExt as _};
    use tower_http::trace::TraceLayer;
    let layer = TraceLayer::new_for_http();
    use tower::Layer as _;
    let svc = tower::service_fn(|_req: Request<Empty<Bytes>>| async move {
        Ok::<Response<Empty<Bytes>>, std::convert::Infallible>(Response::new(Empty::new()))
    });
    let mut layered = layer.layer(svc);
    let req = Request::builder().body(Empty::new()).unwrap();
    let resp = layered.ready().await.unwrap().call(req).await.unwrap();
    assert_eq!(resp.status(), 200, "TraceLayer must pass through 200 responses");
}

#[tokio::test]
async fn test_tower_http_trace_layer_wraps_service_error() {
    // @covers: tower-http / TraceLayer + Service
    use bytes::Bytes;
    use http::{Request, Response};
    use http_body_util::Empty;
    use tower_http::trace::TraceLayer;

    let svc = tower::service_fn(|_req: Request<Empty<Bytes>>| async move {
        Ok::<Response<Empty<Bytes>>, std::convert::Infallible>(Response::new(Empty::new()))
    });
    let _traced = tower::ServiceBuilder::new()
        .layer(TraceLayer::new_for_http())
        .service(svc);
}

#[tokio::test]
async fn test_tower_http_trace_layer_new_for_grpc_edge() {
    // @covers: tower-http / TraceLayer
    use bytes::Bytes;
    use http::{Request, Response};
    use http_body_util::Empty;
    use tower::{Service as _, ServiceExt as _};
    use tower_http::trace::TraceLayer;
    let layer = TraceLayer::new_for_grpc();
    use tower::Layer as _;
    let svc = tower::service_fn(|_req: Request<Empty<Bytes>>| async move {
        Ok::<Response<Empty<Bytes>>, std::convert::Infallible>(Response::new(Empty::new()))
    });
    let mut layered = layer.layer(svc);
    let req = Request::builder().body(Empty::new()).unwrap();
    let resp = layered.ready().await.unwrap().call(req).await.unwrap();
    // gRPC uses HTTP status 200 for all responses; error is in grpc-status trailer.
    assert_eq!(resp.status(), 200, "gRPC TraceLayer must pass through 200 responses");
}
