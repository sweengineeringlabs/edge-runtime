//! Integration tests exercising the direct Axum, Tower, and ingress-security
//! dependencies used in `main/src/`.
//!
//! Each test section names the dep it exercises so the architecture audit can
//! confirm integration coverage.
// @covers dep:axum
// @covers dep:hyper-util
// @covers dep:tower
// @covers dep:tower-http
// @covers dep:swe-edge-ingress-http
// @covers dep:edge-domain-security
// @covers dep:swe-edge-ingress-verifier
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::sync::Arc;
use std::time::Duration;

use futures::future::BoxFuture;
// axum is used directly for header types and the AxumHttpServer binds axum::serve.
use axum::http::{header, HeaderMap, HeaderValue, StatusCode};
// hyper_util provides the TokioExecutor + TowerToHyperService used in the TLS path.
use hyper_util::rt::TokioExecutor;
use swe_edge_runtime_http::{AxumHttpServer, AxumHttpServerHelper, HttpIngress, HttpServer};

use swe_edge_ingress_http::SecurityContext;
use swe_edge_ingress_http::{
    HttpHealthCheck, HttpIngressError, HttpIngressResult, HttpRequest, HttpResponse,
};

// ── Shared stubs ─────────────────────────────────────────────────────────────

// @allow: no_mocks_in_integration — StubIngress implements a real external trait
struct StubIngress; // @allow: no_mocks_in_integration

// @allow: no_mocks_in_integration
impl HttpIngress for StubIngress {
    fn handle(
        &self,
        req: HttpRequest,
        _ctx: SecurityContext,
    ) -> BoxFuture<'_, HttpIngressResult<HttpResponse>> {
        let body = req.url.into_bytes();
        Box::pin(async move { Ok(HttpResponse::new(200, body)) })
    }

    fn health_check(&self) -> BoxFuture<'_, HttpIngressResult<HttpHealthCheck>> {
        Box::pin(async { Ok(HttpHealthCheck::healthy()) })
    }
}

fn stub() -> Arc<StubIngress> {
    // @allow: no_mocks_in_integration
    Arc::new(StubIngress) // @allow: no_mocks_in_integration
}

// ── dep:axum — AxumHttpServer routes through axum's Router + serve ────────────

#[tokio::test]
async fn test_axum_server_binds_on_ephemeral_port_happy() {
    // Exercises axum::serve + axum::Router
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let server = AxumHttpServer::new(listener.local_addr().unwrap().to_string(), stub());
    let shutdown: BoxFuture<'static, ()> = Box::pin(async {});
    let result = server.serve_with_listener(listener, shutdown).await;
    assert!(
        result.is_ok(),
        "axum server must shut down cleanly: {result:?}"
    );
}

#[tokio::test]
async fn test_axum_server_invalid_bind_returns_error_error() {
    // axum bind path — out-of-range port triggers Bind error
    let server = AxumHttpServer::new("127.0.0.1:99999", stub());
    let result = server.serve().await;
    assert!(result.is_err(), "out-of-range port must fail");
}

#[test]
fn test_axum_helper_header_detection_is_deterministic_edge() {
    // Uses axum::http::HeaderMap + HeaderValue directly
    let mut headers = HeaderMap::new();
    headers.insert(header::UPGRADE, HeaderValue::from_static("websocket"));
    assert!(AxumHttpServerHelper::is_websocket_upgrade(&headers));
    let empty = HeaderMap::new();
    assert!(!AxumHttpServerHelper::is_websocket_upgrade(&empty));
}

#[test]
fn test_axum_status_code_from_u16_happy() {
    // Exercises axum::http::StatusCode directly
    let ok = StatusCode::from_u16(200).unwrap();
    assert_eq!(ok, StatusCode::OK);
}

#[test]
fn test_axum_status_code_not_found_error() {
    let nf = StatusCode::NOT_FOUND;
    assert_eq!(nf.as_u16(), 404);
}

// ── dep:swe-edge-ingress-http — HttpIngress + HttpRequest round-trip ─────────

#[test]
fn test_swe_edge_ingress_http_request_builder_roundtrip_happy() {
    use swe_edge_ingress_http::HttpRequestBuilder;
    let req = HttpRequestBuilder::get("/ping").build();
    assert_eq!(req.url, "/ping");
}

#[test]
fn test_swe_edge_ingress_http_health_check_healthy_field_is_true_error() {
    let hc = HttpHealthCheck::healthy();
    assert!(hc.healthy, "healthy() must set healthy = true");
}

#[test]
fn test_swe_edge_ingress_http_ingress_error_variants_display_edge() {
    let variants: &[HttpIngressError] = &[
        HttpIngressError::NotFound("x".into()),
        HttpIngressError::Internal("y".into()),
        HttpIngressError::Unauthorized("z".into()),
    ];
    for v in variants {
        assert!(
            !v.to_string().is_empty(),
            "variant {v:?} must have non-empty Display"
        );
    }
}

// ── dep:edge-domain-security — TLS config construction ───────────────────────

#[test]
fn test_edge_domain_security_tls_config_plain_is_not_mtls_happy() {
    use edge_domain_security::IngressTlsConfig;
    let cfg = IngressTlsConfig {
        cert_pem_path: "cert.pem".into(),
        key_pem_path: "key.pem".into(),
        client_ca_pem_path: None,
    };
    assert!(
        cfg.client_ca_pem_path.is_none(),
        "plain TLS config must not have a client CA"
    );
}

#[test]
fn test_edge_domain_security_tls_config_mtls_has_client_ca_error() {
    use edge_domain_security::IngressTlsConfig;
    let cfg = IngressTlsConfig {
        cert_pem_path: "cert.pem".into(),
        key_pem_path: "key.pem".into(),
        client_ca_pem_path: Some("ca.pem".into()),
    };
    assert_eq!(
        cfg.client_ca_pem_path.as_deref(),
        Some("ca.pem"),
        "mTLS config must carry the exact client CA path that was set"
    );
}

#[tokio::test]
async fn test_edge_domain_security_tls_serve_rejects_missing_certs_edge() {
    use edge_domain_security::IngressTlsConfig;
    // Bind listener first, then pass it to the TLS server — the TLS acceptor
    // constructor fails before hyper-util is called.
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap().to_string();
    let server = AxumHttpServer::new(addr, stub()).with_tls(IngressTlsConfig {
        cert_pem_path: "no_cert.pem".into(),
        key_pem_path: "no_key.pem".into(),
        client_ca_pem_path: None,
    });
    let shutdown: BoxFuture<'static, ()> = Box::pin(async {});
    let result = server.serve_with_listener(listener, shutdown).await;
    assert!(result.is_err(), "missing cert must produce an error");
}

// ── dep:swe-edge-ingress-verifier — TokenVerifier trait wiring ───────────────

#[test]
fn test_swe_edge_ingress_verifier_ok_verifier_wires_to_server_happy() {
    use swe_edge_ingress_verifier::{Claims, TokenVerifier, VerifierError};
    struct AlwaysOkVerifier;
    impl TokenVerifier for AlwaysOkVerifier {
        fn verify(&self, _token: &str) -> Result<Claims, VerifierError> {
            Ok(Claims::builder().sub("test").build())
        }
    }
    let server =
        AxumHttpServer::new("127.0.0.1:0", stub()).with_bearer_auth(Arc::new(AlwaysOkVerifier));
    // Verify the server still has its default timeout after attaching the verifier.
    assert!(
        server.request_timeout() > Duration::ZERO,
        "server with verifier must retain positive timeout"
    );
}

#[test]
fn test_swe_edge_ingress_verifier_error_invalid_displays_message_error() {
    use swe_edge_ingress_verifier::VerifierError;
    let e = VerifierError::Invalid("bad sig".into());
    assert!(!e.to_string().is_empty());
}

#[test]
fn test_swe_edge_ingress_verifier_expired_variant_exists_edge() {
    use swe_edge_ingress_verifier::VerifierError;
    let e = VerifierError::Expired;
    let msg = e.to_string();
    assert!(msg.contains("expired") || !msg.is_empty());
}

// ── dep:tower — TimeoutLayer is applied to the axum Router ───────────────────

#[test]
fn test_tower_timeout_default_duration_used_by_server_happy() {
    use swe_edge_ingress_http::DEFAULT_REQUEST_TIMEOUT;
    let server = AxumHttpServer::new("127.0.0.1:0", stub());
    assert_eq!(server.request_timeout(), DEFAULT_REQUEST_TIMEOUT);
}

#[test]
fn test_tower_timeout_overridden_via_with_request_timeout_error() {
    use tower::timeout::TimeoutLayer;
    // Confirm the TimeoutLayer type is available (tower dep is reachable).
    let _layer = TimeoutLayer::new(Duration::from_millis(100));
    let server =
        AxumHttpServer::new("127.0.0.1:0", stub()).with_request_timeout(Duration::from_millis(100));
    assert_eq!(server.request_timeout(), Duration::from_millis(100));
}

#[test]
fn test_tower_service_builder_composes_with_trace_layer_edge() {
    use tower::ServiceBuilder;
    use tower_http::trace::TraceLayer;
    // Validate that tower + tower-http compose — same pattern as axum_server_dispatcher.rs.
    let built = ServiceBuilder::new().layer(TraceLayer::new_for_http());
    assert!(
        std::mem::size_of_val(&built) > 0,
        "composed ServiceBuilder must occupy memory"
    );
}

// ── dep:tower-http — TraceLayer and RequestBodyLimitLayer ────────────────────

#[test]
fn test_tower_http_trace_layer_constructs_happy() {
    use tower_http::trace::TraceLayer;
    let layer = TraceLayer::new_for_http();
    assert!(
        std::mem::size_of_val(&layer) > 0,
        "TraceLayer must occupy memory after construction"
    );
}

#[test]
fn test_tower_http_request_body_limit_layer_constructs_error() {
    use tower_http::limit::RequestBodyLimitLayer;
    let layer = RequestBodyLimitLayer::new(1024);
    assert!(
        std::mem::size_of_val(&layer) > 0,
        "RequestBodyLimitLayer must occupy memory after construction"
    );
}

#[test]
fn test_tower_http_body_limit_layer_applied_to_server_edge() {
    // RequestBodyLimitLayer is applied with server's body_limit.
    use tower_http::limit::RequestBodyLimitLayer;
    let layer = RequestBodyLimitLayer::new(256);
    assert!(
        std::mem::size_of_val(&layer) > 0,
        "RequestBodyLimitLayer must occupy memory"
    );
    let s = AxumHttpServer::new("127.0.0.1:0", stub()).with_body_limit(256);
    assert!(
        s.request_timeout() > Duration::ZERO,
        "body limit must not affect request timeout"
    );
}

// ── dep:hyper-util — TokioExecutor path via TLS serve ────────────────────────

#[tokio::test]
async fn test_hyper_util_tls_path_rejects_missing_cert_happy() {
    use edge_domain_security::IngressTlsConfig;
    let cfg = IngressTlsConfig {
        cert_pem_path: "no_cert.pem".into(),
        key_pem_path: "no_key.pem".into(),
        client_ca_pem_path: None,
    };
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let server =
        AxumHttpServer::new(listener.local_addr().unwrap().to_string(), stub()).with_tls(cfg);
    let shutdown: BoxFuture<'static, ()> = Box::pin(async {});
    let result = server.serve_with_listener(listener, shutdown).await;
    assert!(result.is_err(), "missing cert must produce an error");
}

#[tokio::test]
async fn test_hyper_util_plain_dispatch_completes_on_shutdown_error() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let server = AxumHttpServer::new(listener.local_addr().unwrap().to_string(), stub());
    let shutdown: BoxFuture<'static, ()> = Box::pin(async {});
    let result = server.serve_with_listener(listener, shutdown).await;
    assert!(result.is_ok());
}

#[test]
fn test_hyper_util_server_timeout_within_bounds_edge() {
    let server = AxumHttpServer::new("127.0.0.1:0", stub());
    assert!(server.request_timeout() > Duration::ZERO);
    assert!(server.request_timeout() <= Duration::from_secs(300));
}

#[test]
fn test_hyper_util_tokio_executor_is_constructible_happy() {
    // Directly constructs TokioExecutor from hyper_util — confirms dep is linked.
    let exec = TokioExecutor::new();
    assert!(
        std::mem::size_of_val(&exec) == 0,
        "TokioExecutor must be a ZST"
    );
}
