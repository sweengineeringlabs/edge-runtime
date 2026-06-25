//! Integration tests for `AxumHttpServer` public API type.
// @covers api/server/types/axum_http_server.rs
#![allow(clippy::unwrap_used)]

use std::sync::Arc;
use std::time::Duration;

use futures::future::BoxFuture;
use swe_edge_ingress_http::{
    HttpHealthCheck, HttpIngressResult, HttpRequest, HttpResponse, SecurityContext,
    DEFAULT_REQUEST_TIMEOUT,
};
use swe_edge_runtime_http::{AxumHttpServer, HttpIngress, HttpServer};

struct OkIngress;

impl HttpIngress for OkIngress {
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

fn server() -> AxumHttpServer {
    AxumHttpServer::new("127.0.0.1:0", Arc::new(OkIngress))
}

// ── new — construction ────────────────────────────────────────────────────────

#[test]
fn test_new_constructs_without_panic_happy() {
    let s = server();
    assert_eq!(
        s.request_timeout(),
        DEFAULT_REQUEST_TIMEOUT,
        "new server must have default timeout"
    );
}

#[test]
fn test_new_default_request_timeout_is_expected_happy() {
    assert_eq!(server().request_timeout(), DEFAULT_REQUEST_TIMEOUT);
}

#[test]
fn test_new_accepts_ipv6_bind_edge() {
    let s = AxumHttpServer::new("[::1]:0", Arc::new(OkIngress));
    assert_eq!(
        s.request_timeout(),
        DEFAULT_REQUEST_TIMEOUT,
        "IPv6 bind must retain default timeout"
    );
}

// ── with_body_limit ───────────────────────────────────────────────────────────

#[test]
fn test_with_body_limit_does_not_panic_happy() {
    let s = server().with_body_limit(1024);
    assert_eq!(
        s.request_timeout(),
        DEFAULT_REQUEST_TIMEOUT,
        "with_body_limit must not affect request timeout"
    );
}

#[test]
fn test_with_body_limit_zero_does_not_panic_error() {
    let s = server().with_body_limit(0);
    assert_eq!(
        s.request_timeout(),
        DEFAULT_REQUEST_TIMEOUT,
        "zero body limit must not affect request timeout"
    );
}

#[test]
fn test_with_body_limit_combined_with_timeout_edge() {
    let s = server()
        .with_body_limit(512)
        .with_request_timeout(Duration::from_millis(10));
    assert_eq!(s.request_timeout(), Duration::from_millis(10));
}

// ── with_request_timeout ──────────────────────────────────────────────────────

#[test]
fn test_with_request_timeout_50ms_happy() {
    let s = server().with_request_timeout(Duration::from_millis(50));
    assert_eq!(s.request_timeout(), Duration::from_millis(50));
}

#[test]
fn test_with_request_timeout_zero_accepted_error() {
    let s = server().with_request_timeout(Duration::ZERO);
    assert_eq!(s.request_timeout(), Duration::ZERO);
}

#[test]
fn test_with_request_timeout_large_value_edge() {
    let large = Duration::from_secs(3600);
    let s = server().with_request_timeout(large);
    assert_eq!(s.request_timeout(), large);
}

// ── with_tls — construction does not panic ───────────────────────────────────

#[test]
fn test_with_tls_does_not_panic_happy() {
    use swe_edge_ingress_tls::IngressTlsConfig;
    let s = server().with_tls(IngressTlsConfig::tls("c.pem", "k.pem"));
    assert_eq!(
        s.request_timeout(),
        DEFAULT_REQUEST_TIMEOUT,
        "with_tls must not affect request timeout"
    );
}

#[test]
fn test_with_mtls_does_not_panic_error() {
    use swe_edge_ingress_tls::IngressTlsConfig;
    let s = server().with_tls(IngressTlsConfig::mtls("c.pem", "k.pem", "ca.pem"));
    assert_eq!(
        s.request_timeout(),
        DEFAULT_REQUEST_TIMEOUT,
        "with_mtls must not affect request timeout"
    );
}

#[test]
fn test_tls_serve_rejects_missing_cert_edge() {
    use swe_edge_ingress_tls::IngressTlsConfig;
    let result = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
            let s = server().with_tls(IngressTlsConfig::tls("no.pem", "no.pem"));
            let shutdown: BoxFuture<'static, ()> = Box::pin(async {});
            s.serve_with_listener(listener, shutdown).await
        });
    assert!(result.is_err());
}

// ── with_bearer_auth ──────────────────────────────────────────────────────────

#[test]
fn test_with_bearer_auth_does_not_panic_happy() {
    use swe_edge_ingress_verifier::{Claims, TokenVerifier, VerifierError};
    struct DenyAll;
    impl TokenVerifier for DenyAll {
        fn verify(&self, _: &str) -> Result<Claims, VerifierError> {
            Err(VerifierError::Invalid("no".into()))
        }
    }
    let s = server().with_bearer_auth(Arc::new(DenyAll));
    assert_eq!(
        s.request_timeout(),
        DEFAULT_REQUEST_TIMEOUT,
        "with_bearer_auth must not affect request timeout"
    );
}

#[test]
fn test_without_bearer_auth_timeout_still_default_error() {
    assert_eq!(server().request_timeout(), DEFAULT_REQUEST_TIMEOUT);
}

#[test]
fn test_with_stream_handler_does_not_panic_edge() {
    use swe_edge_ingress_http::{HttpIngressError, HttpStream, SseStream, WsChannel};
    struct NoopStream;
    impl HttpStream for NoopStream {
        fn handle_sse(
            &self,
            _: HttpRequest,
            _: SecurityContext,
        ) -> BoxFuture<'_, swe_edge_ingress_http::HttpIngressResult<SseStream>> {
            Box::pin(async { Err(HttpIngressError::MethodNotAllowed("no".into())) })
        }
        fn handle_websocket(
            &self,
            _: HttpRequest,
            _: SecurityContext,
            _: WsChannel,
        ) -> BoxFuture<'_, swe_edge_ingress_http::HttpIngressResult<()>> {
            Box::pin(async { Ok(()) })
        }
    }
    let s = server().with_stream_handler(Arc::new(NoopStream));
    assert_eq!(
        s.request_timeout(),
        DEFAULT_REQUEST_TIMEOUT,
        "with_stream_handler must not affect request timeout"
    );
}

// ── serve ─────────────────────────────────────────────────────────────────────

#[tokio::test]
async fn test_serve_invalid_port_returns_bind_error_happy() {
    let result = AxumHttpServer::new("127.0.0.1:99999", Arc::new(OkIngress))
        .serve()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_serve_with_listener_immediate_shutdown_returns_ok_error() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let shutdown: BoxFuture<'static, ()> = Box::pin(async {});
    let result = server().serve_with_listener(listener, shutdown).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_serve_with_shutdown_immediate_returns_ok_edge() {
    let shutdown: BoxFuture<'static, ()> = Box::pin(async {});
    let result = AxumHttpServer::new("127.0.0.1:0", Arc::new(OkIngress))
        .serve_with_shutdown(shutdown)
        .await;
    assert!(result.is_ok());
}
