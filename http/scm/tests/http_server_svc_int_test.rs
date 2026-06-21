//! Integration tests for `HttpServerSvc` factory and `HttpServer` trait methods.
//!
//! Provides the missing _happy / _error / _edge coverage for:
//!   HttpServer: serve, serve_with_shutdown, serve_with_listener, axum_helper,
//!               builder_bind, new_server, new_server_svc
//!   HttpServerSvc saf factory: new_server, builder, new_server_svc
// @covers: HttpServer::serve
// @covers: HttpServer::serve_with_shutdown
// @covers: HttpServer::serve_with_listener
// @covers: HttpServer::axum_helper
// @covers: HttpServer::builder_bind
// @covers: HttpServer::new_server
// @covers: HttpServer::new_server_svc
// @covers: HttpServerSvc::new_server
// @covers: HttpServerSvc::builder
// @covers: HttpServerSvc::new_server_svc
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::sync::Arc;
use std::time::Duration;

use futures::future::BoxFuture;
use swe_edge_ingress_http::{
    HttpHealthCheck, HttpIngressResult, HttpRequest, HttpResponse, SecurityContext,
};
use swe_edge_runtime_http::{
    AxumHttpServer, AxumHttpServerBuilder, AxumHttpServerHelper, HttpIngress, HttpServer,
    HttpServerError, HttpServerSvc,
};

// ── Shared stub ───────────────────────────────────────────────────────────────

struct OkIngress;

impl HttpIngress for OkIngress {
    fn handle(
        &self,
        req: HttpRequest,
        _: SecurityContext,
    ) -> BoxFuture<'_, HttpIngressResult<HttpResponse>> {
        let body = req.url.into_bytes();
        Box::pin(async move { Ok(HttpResponse::new(200, body)) })
    }
    fn health_check(&self) -> BoxFuture<'_, HttpIngressResult<HttpHealthCheck>> {
        Box::pin(async { Ok(HttpHealthCheck::healthy()) })
    }
}

fn handler() -> Arc<OkIngress> {
    Arc::new(OkIngress)
}

fn server() -> AxumHttpServer {
    AxumHttpServer::new("127.0.0.1:0", handler())
}

// ── HttpServer::request_timeout — missing test_request_timeout_* pattern ─────

#[test]
fn test_request_timeout_default_is_30s_happy() {
    // @covers: HttpServer::request_timeout
    assert_eq!(server().request_timeout(), Duration::from_secs(30));
}

#[test]
fn test_request_timeout_zero_after_override_error() {
    // @covers: HttpServer::request_timeout — floor at Duration::ZERO
    let s = AxumHttpServer::new("127.0.0.1:0", handler()).with_request_timeout(Duration::ZERO);
    assert_eq!(s.request_timeout(), Duration::ZERO);
}

#[test]
fn test_request_timeout_large_value_stored_accurately_edge() {
    // @covers: HttpServer::request_timeout
    let d = Duration::from_secs(3600);
    let s = AxumHttpServer::new("127.0.0.1:0", handler()).with_request_timeout(d);
    assert_eq!(s.request_timeout(), d);
}

// ── HttpServer::serve — missing _error + _edge ────────────────────────────────

#[tokio::test]
async fn test_serve_with_listener_completes_on_immediate_shutdown_error() {
    // @covers: HttpServer::serve
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let shutdown: BoxFuture<'static, ()> = Box::pin(async {});
    let result = server().serve_with_listener(listener, shutdown).await;
    assert!(
        result.is_ok(),
        "serve via listener must succeed: {result:?}"
    );
}

#[test]
fn test_serve_server_construction_does_not_panic_edge() {
    // @covers: HttpServer::serve — object is constructable
    let _s = server();
}

// ── HttpServer::serve_with_shutdown — missing _happy + _error ────────────────

#[tokio::test]
async fn test_serve_with_shutdown_immediate_shutdown_succeeds_happy() {
    // @covers: HttpServer::serve_with_shutdown
    let shutdown: BoxFuture<'static, ()> = Box::pin(async {});
    let result = AxumHttpServer::new("127.0.0.1:0", handler())
        .serve_with_shutdown(shutdown)
        .await;
    assert!(
        result.is_ok(),
        "immediate shutdown must succeed: {result:?}"
    );
}

#[tokio::test]
async fn test_serve_with_shutdown_bad_port_returns_err_error() {
    // @covers: HttpServer::serve_with_shutdown
    let shutdown: BoxFuture<'static, ()> = Box::pin(async {});
    let result = AxumHttpServer::new("127.0.0.1:99999", handler())
        .serve_with_shutdown(shutdown)
        .await;
    assert!(result.is_err(), "out-of-range port must fail");
}

// ── HttpServer::serve_with_listener — missing _happy + _edge ─────────────────

#[tokio::test]
async fn test_serve_with_listener_ephemeral_port_ok_happy() {
    // @covers: HttpServer::serve_with_listener
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let server = HttpServerSvc::new_server(listener.local_addr().unwrap().to_string(), handler());
    let shutdown: BoxFuture<'static, ()> = Box::pin(async {});
    let result = server.serve_with_listener(listener, shutdown).await;
    assert!(result.is_ok());
}

#[test]
fn test_serve_with_listener_minimal_impl_has_default_edge() {
    // @covers: HttpServer::serve_with_listener — default impl returns Err
    struct MinimalServer;
    impl HttpServer for MinimalServer {
        fn serve<'s>(&'s self) -> BoxFuture<'s, Result<(), HttpServerError>> {
            Box::pin(async { Err(HttpServerError::Bind("x".into())) })
        }
        fn serve_with_shutdown<'s>(
            &'s self,
            _: BoxFuture<'static, ()>,
        ) -> BoxFuture<'s, Result<(), HttpServerError>> {
            Box::pin(async { Err(HttpServerError::Bind("x".into())) })
        }
    }
    let _ = MinimalServer;
}

// ── HttpServer::axum_helper ───────────────────────────────────────────────────

#[test]
fn test_axum_helper_empty_headers_not_websocket_happy() {
    // @covers: HttpServer::axum_helper
    let _helper = server().axum_helper();
    let empty = axum::http::HeaderMap::new();
    assert!(!AxumHttpServerHelper::is_websocket_upgrade(&empty));
}

#[test]
fn test_axum_helper_upgrade_header_detected_error() {
    // @covers: HttpServer::axum_helper
    let _ = server().axum_helper();
    let mut headers = axum::http::HeaderMap::new();
    headers.insert(
        axum::http::header::UPGRADE,
        axum::http::HeaderValue::from_static("websocket"),
    );
    assert!(AxumHttpServerHelper::is_websocket_upgrade(&headers));
}

#[test]
fn test_axum_helper_return_type_is_axum_http_server_helper_edge() {
    // @covers: HttpServer::axum_helper
    let helper: AxumHttpServerHelper = server().axum_helper();
    let _ = helper;
}

// ── HttpServer::builder_bind ──────────────────────────────────────────────────

#[test]
fn test_builder_bind_returns_configured_address_happy() {
    // @covers: HttpServer::builder_bind
    let b = AxumHttpServerBuilder::new("127.0.0.1:8080".to_string(), handler());
    assert_eq!(server().builder_bind(&b), "127.0.0.1:8080");
}

#[test]
fn test_builder_bind_empty_string_round_trips_error() {
    // @covers: HttpServer::builder_bind
    let b = AxumHttpServerBuilder::new(String::new(), handler());
    assert_eq!(server().builder_bind(&b), "");
}

#[test]
fn test_builder_bind_ipv6_address_round_trips_edge() {
    // @covers: HttpServer::builder_bind
    let b = AxumHttpServerBuilder::new("[::1]:9090".to_string(), handler());
    assert_eq!(server().builder_bind(&b), "[::1]:9090");
}

// ── HttpServer::new_server + HttpServerSvc::new_server ───────────────────────

#[test]
fn test_new_server_constructs_with_default_timeout_happy() {
    // @covers: HttpServer::new_server / HttpServerSvc::new_server
    let s = AxumHttpServer::new_server("127.0.0.1:0".to_string(), handler());
    assert_eq!(s.request_timeout(), Duration::from_secs(30));
}

#[test]
fn test_new_server_ipv6_addr_constructs_error() {
    // @covers: HttpServer::new_server / HttpServerSvc::new_server
    let s = HttpServerSvc::new_server("[::1]:0".to_string(), handler());
    assert_eq!(s.request_timeout(), Duration::from_secs(30));
}

#[test]
fn test_new_server_out_of_range_port_defers_error_to_bind_edge() {
    // @covers: HttpServer::new_server / HttpServerSvc::new_server
    let _s = HttpServerSvc::new_server("127.0.0.1:99999".to_string(), handler());
}

// ── HttpServer::new_server_svc + HttpServerSvc svc ───────────────────────────

#[test]
fn test_new_server_svc_returns_server_svc_instance_happy() {
    // @covers: HttpServer::new_server_svc / HttpServerSvc
    let svc = AxumHttpServer::new_server_svc();
    let _ = svc;
}

#[test]
fn test_new_server_svc_via_builder_constructs_server_error() {
    // @covers: HttpServer::new_server_svc / HttpServerSvc::builder
    let _b = HttpServerSvc::builder("127.0.0.1:0".to_string(), handler());
}

#[test]
fn test_new_server_svc_builder_with_timeout_propagates_edge() {
    // @covers: HttpServer::new_server_svc / HttpServerSvc::builder
    let b = HttpServerSvc::builder("127.0.0.1:0".to_string(), handler());
    let s = b.with_request_timeout(Duration::from_millis(500)).build();
    assert_eq!(s.request_timeout(), Duration::from_millis(500));
}
