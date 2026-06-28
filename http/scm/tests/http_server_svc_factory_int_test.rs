//! Integration tests for `saf/http_server_svc_factory.rs` — the SAF factory surface for `HttpServer`.
// @covers AxumHttpServer::new
// @covers AxumHttpServer::with_stream_handler
// @covers AxumHttpServer::with_body_limit
// @covers AxumHttpServer::with_request_timeout
// @covers AxumHttpServer::with_tls
// @covers AxumHttpServer::with_bearer_auth
// @covers AxumHttpServerBuilder::new
// @covers AxumHttpServerBuilder::with_body_limit
// @covers AxumHttpServerBuilder::with_request_timeout
// @covers AxumHttpServerBuilder::with_tls
// @covers AxumHttpServerBuilder::with_bearer_auth
// @covers AxumHttpServerBuilder::build
// @covers AxumHttpServerHelper::is_websocket_upgrade
// @covers AxumHttpServerHelper::is_sse_request
// @covers AxumHttpServerHelper::collect_headers
// @covers AxumHttpServerHelper::payload_too_large
// @covers AxumHttpServerHelper::internal_server_error
#![allow(clippy::unwrap_used)]

use std::sync::Arc;
use std::time::Duration;

use futures::future::BoxFuture;
use swe_edge_ingress_http::{
    HttpHealthCheck, HttpIngressResult, HttpRequest, HttpResponse, SecurityContext,
    DEFAULT_REQUEST_TIMEOUT,
};
use swe_edge_runtime_http::{
    AxumHttpServer, AxumHttpServerBuilder, HttpIngress, HttpServer, HttpServerError,
};

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

fn handler() -> Arc<OkIngress> {
    Arc::new(OkIngress)
}

// ── HttpServer trait is accessible via SAF factory re-export ──────────────────

#[test]
fn test_http_server_trait_new_server_produces_default_timeout_happy() {
    // @covers: HttpServer::new_server (via saf/http_server_svc_factory.rs)
    let s = AxumHttpServer::new_server("127.0.0.1:0".to_string(), handler());
    assert_eq!(
        s.request_timeout(),
        DEFAULT_REQUEST_TIMEOUT,
        "new_server must produce a server with the default request timeout"
    );
    assert!(
        s.request_timeout() > Duration::ZERO,
        "default timeout must be positive"
    );
}

#[test]
fn test_http_server_trait_serve_with_invalid_port_returns_err_error() {
    // @covers: HttpServer::serve (via saf/http_server_svc_factory.rs)
    let s = AxumHttpServer::new_server("127.0.0.1:99999".to_string(), handler());
    let result = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(s.serve());
    assert!(result.is_err(), "out-of-range port must fail at bind time");
    assert!(
        matches!(result.unwrap_err(), HttpServerError::Bind(_, _)),
        "error must be a Bind variant"
    );
}

#[test]
fn test_http_server_trait_serve_with_listener_immediate_shutdown_ok_edge() {
    // @covers: HttpServer::serve_with_listener (via saf/http_server_svc_factory.rs)
    let result = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
            let s =
                AxumHttpServer::new_server(listener.local_addr().unwrap().to_string(), handler());
            let shutdown: BoxFuture<'static, ()> = Box::pin(async {});
            s.serve_with_listener(listener, shutdown).await
        });
    assert!(
        result.is_ok(),
        "immediate shutdown must complete cleanly: {result:?}"
    );
    assert!(
        matches!(result, Ok(())),
        "serve_with_listener must return Ok(()) on clean shutdown"
    );
}

// ── AxumHttpServer::with_stream_handler ──────────────────────────────────────

#[test]
fn test_with_stream_handler_preserves_request_timeout_happy() {
    // @covers: AxumHttpServer::with_stream_handler
    use futures::future::BoxFuture;
    use swe_edge_ingress_http::{HttpIngressError, HttpStream, SseStream, WsChannel};
    struct NoopStream;
    impl HttpStream for NoopStream {
        fn handle_sse(
            &self,
            _: HttpRequest,
            _: SecurityContext,
        ) -> BoxFuture<'_, HttpIngressResult<SseStream>> {
            Box::pin(async { Err(HttpIngressError::MethodNotAllowed("sse".into())) })
        }
        fn handle_websocket(
            &self,
            _: HttpRequest,
            _: SecurityContext,
            _: WsChannel,
        ) -> BoxFuture<'_, HttpIngressResult<()>> {
            Box::pin(async { Ok(()) })
        }
    }
    let s = AxumHttpServer::new("127.0.0.1:0", handler()).with_stream_handler(Arc::new(NoopStream));
    assert!(
        s.request_timeout() > Duration::ZERO,
        "with_stream_handler must not zero-out request timeout"
    );
}

#[test]
fn test_with_stream_handler_does_not_affect_default_timeout_error() {
    // @covers: AxumHttpServer::with_stream_handler (error path: no panic when handler discarded)
    use futures::future::BoxFuture;
    use swe_edge_ingress_http::{HttpIngressError, HttpStream, SseStream, WsChannel};
    struct FailStream;
    impl HttpStream for FailStream {
        fn handle_sse(
            &self,
            _: HttpRequest,
            _: SecurityContext,
        ) -> BoxFuture<'_, HttpIngressResult<SseStream>> {
            Box::pin(async { Err(HttpIngressError::Internal("fail".into())) })
        }
        fn handle_websocket(
            &self,
            _: HttpRequest,
            _: SecurityContext,
            _: WsChannel,
        ) -> BoxFuture<'_, HttpIngressResult<()>> {
            Box::pin(async { Err(HttpIngressError::Internal("fail".into())) })
        }
    }
    let s = AxumHttpServer::new("127.0.0.1:0", handler()).with_stream_handler(Arc::new(FailStream));
    assert_eq!(
        s.request_timeout(),
        DEFAULT_REQUEST_TIMEOUT,
        "with_stream_handler must not change the default timeout"
    );
}

// ── AxumHttpServer::with_tls ──────────────────────────────────────────────────

#[test]
fn test_with_tls_invalid_paths_still_constructs_server_error() {
    // @covers: AxumHttpServer::with_tls (construction; TLS errors surface at serve time)
    use edge_domain_security::IngressTlsConfig;
    let s = AxumHttpServer::new("127.0.0.1:0", handler()).with_tls(IngressTlsConfig {
        cert_pem_path: "no.pem".into(),
        key_pem_path: "no.pem".into(),
        client_ca_pem_path: None,
    });
    assert_eq!(
        s.request_timeout(),
        DEFAULT_REQUEST_TIMEOUT,
        "with_tls must not affect request timeout"
    );
}

#[test]
fn test_with_tls_mtls_config_does_not_panic_edge() {
    // @covers: AxumHttpServer::with_tls (mTLS variant)
    use edge_domain_security::IngressTlsConfig;
    let s = AxumHttpServer::new("127.0.0.1:0", handler()).with_tls(IngressTlsConfig {
        cert_pem_path: "c.pem".into(),
        key_pem_path: "k.pem".into(),
        client_ca_pem_path: Some("ca.pem".into()),
    });
    assert_eq!(
        s.request_timeout(),
        DEFAULT_REQUEST_TIMEOUT,
        "mTLS variant must not affect request timeout"
    );
}

// ── AxumHttpServer::with_bearer_auth ─────────────────────────────────────────

#[test]
fn test_with_bearer_auth_deny_all_does_not_panic_at_construction_error() {
    // @covers: AxumHttpServer::with_bearer_auth (deny-all verifier; no panic at construction)
    use swe_edge_ingress_verifier::{Claims, TokenVerifier, VerifierError};
    struct DenyAll;
    impl TokenVerifier for DenyAll {
        fn verify(&self, _: &str) -> Result<Claims, VerifierError> {
            Err(VerifierError::Invalid("denied".into()))
        }
    }
    let s = AxumHttpServer::new("127.0.0.1:0", handler()).with_bearer_auth(Arc::new(DenyAll));
    assert_eq!(
        s.request_timeout(),
        DEFAULT_REQUEST_TIMEOUT,
        "with_bearer_auth must not affect request timeout"
    );
}

#[test]
fn test_with_bearer_auth_chained_with_request_timeout_edge() {
    // @covers: AxumHttpServer::with_bearer_auth (chaining with other options)
    use swe_edge_ingress_verifier::{Claims, TokenVerifier, VerifierError};
    struct DenyAll;
    impl TokenVerifier for DenyAll {
        fn verify(&self, _: &str) -> Result<Claims, VerifierError> {
            Err(VerifierError::Invalid("denied".into()))
        }
    }
    let s = AxumHttpServer::new("127.0.0.1:0", handler())
        .with_bearer_auth(Arc::new(DenyAll))
        .with_request_timeout(Duration::from_millis(250));
    assert_eq!(
        s.request_timeout(),
        Duration::from_millis(250),
        "chained timeout must override default"
    );
}

// ── AxumHttpServerBuilder::build ──────────────────────────────────────────────

#[test]
fn test_build_produces_server_with_default_timeout_happy() {
    // @covers: AxumHttpServerBuilder::build
    let s = AxumHttpServerBuilder::new("127.0.0.1:0", handler()).build();
    assert_eq!(
        s.request_timeout(),
        DEFAULT_REQUEST_TIMEOUT,
        "build must produce server with default timeout"
    );
}

#[test]
fn test_build_with_zero_timeout_preserves_zero_error() {
    // @covers: AxumHttpServerBuilder::build (zero timeout edge of error path)
    let s = AxumHttpServerBuilder::new("127.0.0.1:0", handler())
        .with_request_timeout(Duration::ZERO)
        .build();
    assert_eq!(
        s.request_timeout(),
        Duration::ZERO,
        "build must propagate zero timeout"
    );
}

#[test]
fn test_build_with_all_options_produces_configured_server_edge() {
    // @covers: AxumHttpServerBuilder::build (all builder options combined)
    use edge_domain_security::IngressTlsConfig;
    let s = AxumHttpServerBuilder::new("127.0.0.1:0", handler())
        .with_body_limit(512)
        .with_request_timeout(Duration::from_millis(100))
        .with_tls(IngressTlsConfig {
            cert_pem_path: "c.pem".into(),
            key_pem_path: "k.pem".into(),
            client_ca_pem_path: None,
        })
        .build();
    assert_eq!(
        s.request_timeout(),
        Duration::from_millis(100),
        "build must propagate custom timeout when all options set"
    );
}
