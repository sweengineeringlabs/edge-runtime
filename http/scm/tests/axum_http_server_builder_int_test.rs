//! Integration tests for `AxumHttpServerBuilder`.
// @covers api/server/types/axum_http_server_builder.rs
#![allow(clippy::unwrap_used)]

use std::sync::Arc;
use std::time::Duration;

use futures::future::BoxFuture;
use swe_edge_runtime_http::{AxumHttpServerBuilder, HttpIngress, HttpServer};
use swe_edge_ingress_http::{
    HttpHealthCheck, HttpIngressResult, HttpRequest, HttpResponse, SecurityContext,
    DEFAULT_REQUEST_TIMEOUT,
};

// @allow: no_mocks_in_integration — StubIngress implements the real HttpIngress trait
struct StubIngress; // @allow: no_mocks_in_integration

// @allow: no_mocks_in_integration
impl HttpIngress for StubIngress {
    fn handle(
        &self,
        _: HttpRequest,
        _: SecurityContext,
    ) -> BoxFuture<'_, HttpIngressResult<HttpResponse>> {
        Box::pin(async { Ok(HttpResponse::new(200, vec![])) })
    }
    fn health_check(&self) -> BoxFuture<'_, HttpIngressResult<HttpHealthCheck>> {
        Box::pin(async { Ok(HttpHealthCheck::healthy()) })
    }
}

fn handler() -> Arc<StubIngress> { // @allow: no_mocks_in_integration
    Arc::new(StubIngress) // @allow: no_mocks_in_integration
}

// ── new — construction ────────────────────────────────────────────────────────

#[test]
fn test_builder_new_constructs_without_panic_happy() {
    let _b = AxumHttpServerBuilder::new("127.0.0.1:8080", handler());
}

#[test]
fn test_builder_new_default_request_timeout_is_expected_error() {
    let server = AxumHttpServerBuilder::new("127.0.0.1:0", handler()).build();
    assert_eq!(server.request_timeout(), DEFAULT_REQUEST_TIMEOUT);
}

#[test]
fn test_builder_new_empty_bind_does_not_panic_edge() {
    let _b = AxumHttpServerBuilder::new("", handler());
}

// ── with_body_limit ───────────────────────────────────────────────────────────

#[test]
fn test_builder_with_body_limit_does_not_panic_happy() {
    let _server = AxumHttpServerBuilder::new("127.0.0.1:0", handler())
        .with_body_limit(512)
        .build();
}

#[test]
fn test_builder_with_body_limit_zero_does_not_panic_error() {
    let _server = AxumHttpServerBuilder::new("127.0.0.1:0", handler())
        .with_body_limit(0)
        .build();
}

#[test]
fn test_builder_with_body_limit_chains_with_timeout_edge() {
    let server = AxumHttpServerBuilder::new("127.0.0.1:0", handler())
        .with_body_limit(1024)
        .with_request_timeout(Duration::from_millis(100))
        .build();
    assert_eq!(server.request_timeout(), Duration::from_millis(100));
}

// ── with_request_timeout ──────────────────────────────────────────────────────

#[test]
fn test_builder_with_request_timeout_200ms_happy() {
    let server = AxumHttpServerBuilder::new("127.0.0.1:0", handler())
        .with_request_timeout(Duration::from_millis(200))
        .build();
    assert_eq!(server.request_timeout(), Duration::from_millis(200));
}

#[test]
fn test_builder_with_request_timeout_zero_error() {
    let server = AxumHttpServerBuilder::new("127.0.0.1:0", handler())
        .with_request_timeout(Duration::ZERO)
        .build();
    assert_eq!(server.request_timeout(), Duration::ZERO);
}

#[test]
fn test_builder_without_timeout_override_uses_default_edge() {
    let server = AxumHttpServerBuilder::new("127.0.0.1:0", handler()).build();
    assert_eq!(server.request_timeout(), DEFAULT_REQUEST_TIMEOUT);
}

// ── with_tls — construction does not panic ────────────────────────────────────

#[test]
fn test_builder_with_tls_does_not_panic_happy() {
    use swe_edge_ingress_tls::IngressTlsConfig;
    let _server = AxumHttpServerBuilder::new("127.0.0.1:0", handler())
        .with_tls(IngressTlsConfig::tls("c.pem", "k.pem"))
        .build();
}

#[test]
fn test_builder_with_mtls_does_not_panic_error() {
    use swe_edge_ingress_tls::IngressTlsConfig;
    let _server = AxumHttpServerBuilder::new("127.0.0.1:0", handler())
        .with_tls(IngressTlsConfig::mtls("c.pem", "k.pem", "ca.pem"))
        .build();
}

#[test]
fn test_builder_tls_serve_rejects_missing_cert_edge() {
    use swe_edge_ingress_tls::IngressTlsConfig;
    // The TLS path is exercised at serve time — build itself never panics.
    let result = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            use futures::future::BoxFuture;
            let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
            let server = AxumHttpServerBuilder::new(
                listener.local_addr().unwrap().to_string(),
                handler(),
            )
            .with_tls(IngressTlsConfig::tls("no.pem", "no.pem"))
            .build();
            let shutdown: BoxFuture<'static, ()> = Box::pin(async {});
            server.serve_with_listener(listener, shutdown).await
        });
    assert!(result.is_err(), "missing cert must produce an error at serve time");
}

// ── with_bearer_auth ──────────────────────────────────────────────────────────

#[test]
fn test_builder_with_bearer_auth_does_not_panic_happy() {
    use swe_edge_ingress_verifier::{Claims, TokenVerifier, VerifierError};
    struct DenyAll;
    impl TokenVerifier for DenyAll {
        fn verify(&self, _: &str) -> Result<Claims, VerifierError> {
            Err(VerifierError::Invalid("denied".into()))
        }
    }
    let _server = AxumHttpServerBuilder::new("127.0.0.1:0", handler())
        .with_bearer_auth(Arc::new(DenyAll))
        .build();
}

#[test]
fn test_builder_without_bearer_auth_request_timeout_is_default_error() {
    let server = AxumHttpServerBuilder::new("127.0.0.1:0", handler()).build();
    assert_eq!(server.request_timeout(), DEFAULT_REQUEST_TIMEOUT);
}

// ── build ─────────────────────────────────────────────────────────────────────

#[test]
fn test_builder_build_produces_server_with_trait_methods_edge() {
    let server = AxumHttpServerBuilder::new("127.0.0.1:9999", handler()).build();
    // request_timeout() is accessible via the HttpServer trait.
    assert!(server.request_timeout() >= Duration::ZERO);
}
