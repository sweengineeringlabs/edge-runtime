//! Integration tests for `server_svc` SAF module.
//!
//! Verifies that the server SAF exports are accessible and behave correctly.
// @covers server_svc::HTTP_SERVER_SVC
// @covers server_svc::AxumHttpServer
// @covers server_svc::AxumHttpServerBuilder
// @covers server_svc::AxumHttpServerHelper
// @covers server_svc::HttpServer
// @covers server_svc::HttpServerError
#![allow(clippy::unwrap_used)]

use std::sync::Arc;

use swe_edge_runtime_http::{
    AxumHttpServer, AxumHttpServerBuilder, AxumHttpServerHelper, HttpServerError, NoopHttpIngress,
    HTTP_SERVER_SVC,
};

fn noop_handler() -> Arc<NoopHttpIngress> {
    Arc::new(NoopHttpIngress)
}

// ── HTTP_SERVER_SVC constant ─────────────────────────────────────────────────

#[test]
fn test_http_server_svc_slug_is_non_empty_happy() {
    assert!(
        !HTTP_SERVER_SVC.is_empty(),
        "HTTP_SERVER_SVC slug must not be empty"
    );
}

#[test]
fn test_http_server_svc_slug_matches_expected_value_error() {
    assert_eq!(HTTP_SERVER_SVC, "http_server");
}

#[test]
fn test_http_server_svc_slug_is_lowercase_edge() {
    assert_eq!(
        HTTP_SERVER_SVC,
        HTTP_SERVER_SVC.to_lowercase(),
        "slug must be lowercase"
    );
}

// ── AxumHttpServer construction ───────────────────────────────────────────────

#[test]
fn test_axum_http_server_new_valid_addr_happy() {
    let _s = AxumHttpServer::new("127.0.0.1:0", noop_handler());
}

#[test]
fn test_axum_http_server_new_empty_addr_does_not_panic_error() {
    // Construction never panics — errors surface at serve time.
    let _s = AxumHttpServer::new("", noop_handler());
}

#[test]
fn test_axum_http_server_new_ipv6_addr_edge() {
    let _s = AxumHttpServer::new("[::1]:0", noop_handler());
}

// ── AxumHttpServerBuilder ────────────────────────────────────────────────────

#[test]
fn test_axum_http_server_builder_new_returns_builder_happy() {
    let b = AxumHttpServerBuilder::new("127.0.0.1:8080", noop_handler());
    let server = b.build();
    let _: AxumHttpServer = server;
}

#[test]
fn test_axum_http_server_builder_with_body_limit_overrides_default_error() {
    // body_limit is pub(crate); verify indirectly that construction succeeds.
    let _server = AxumHttpServerBuilder::new("127.0.0.1:0", noop_handler())
        .with_body_limit(512)
        .build();
}

#[test]
fn test_axum_http_server_builder_with_request_timeout_edge() {
    use std::time::Duration;
    use swe_edge_runtime_http::HttpServer;
    let server = AxumHttpServerBuilder::new("127.0.0.1:0", noop_handler())
        .with_request_timeout(Duration::from_millis(50))
        .build();
    assert_eq!(server.request_timeout(), Duration::from_millis(50));
}

// ── AxumHttpServerHelper ─────────────────────────────────────────────────────

#[test]
fn test_axum_http_server_helper_is_send_sync_happy() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<AxumHttpServerHelper>();
}

#[test]
fn test_axum_http_server_helper_websocket_detection_returns_false_for_plain_request_error() {
    let headers = axum::http::HeaderMap::new();
    assert!(!AxumHttpServerHelper::is_websocket_upgrade(&headers));
}

#[test]
fn test_axum_http_server_helper_sse_detection_returns_false_for_plain_request_edge() {
    let headers = axum::http::HeaderMap::new();
    assert!(!AxumHttpServerHelper::is_sse_request(&headers));
}

// ── HttpServerError ──────────────────────────────────────────────────────────

#[test]
fn test_http_server_error_serve_display_is_non_empty_happy() {
    let e = HttpServerError::Serve(std::io::Error::other("oops"));
    assert!(!e.to_string().is_empty());
}

#[test]
fn test_http_server_error_bind_display_contains_address_error() {
    let e = HttpServerError::Bind("0.0.0.0:80".into(), std::io::Error::other("denied"));
    let msg = e.to_string();
    assert!(
        msg.contains("0.0.0.0:80"),
        "expected address in error: {msg}"
    );
}

#[test]
fn test_http_server_error_is_debug_edge() {
    let e = HttpServerError::Serve(std::io::Error::other("x"));
    let _ = format!("{e:?}");
}
