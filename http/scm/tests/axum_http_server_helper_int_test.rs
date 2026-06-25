//! Integration tests for `AxumHttpServerHelper` public helpers.
// @covers AxumHttpServerHelper::is_websocket_upgrade
// @covers AxumHttpServerHelper::is_sse_request
// @covers AxumHttpServerHelper::collect_headers
// @covers AxumHttpServerHelper::payload_too_large
// @covers AxumHttpServerHelper::internal_server_error
#![allow(clippy::unwrap_used)]

use swe_edge_runtime_http::AxumHttpServerHelper;

// ── is_websocket_upgrade ──────────────────────────────────────────────────────

#[test]
fn test_is_websocket_upgrade_with_websocket_header_returns_true_happy() {
    let mut headers = axum::http::HeaderMap::new();
    headers.insert(
        axum::http::header::UPGRADE,
        axum::http::HeaderValue::from_static("websocket"),
    );
    assert!(AxumHttpServerHelper::is_websocket_upgrade(&headers));
}

#[test]
fn test_is_websocket_upgrade_with_no_header_returns_false_error() {
    let headers = axum::http::HeaderMap::new();
    assert!(!AxumHttpServerHelper::is_websocket_upgrade(&headers));
}

#[test]
fn test_is_websocket_upgrade_with_non_websocket_upgrade_returns_false_edge() {
    let mut headers = axum::http::HeaderMap::new();
    headers.insert(
        axum::http::header::UPGRADE,
        axum::http::HeaderValue::from_static("h2c"),
    );
    assert!(!AxumHttpServerHelper::is_websocket_upgrade(&headers));
}

// ── is_sse_request ────────────────────────────────────────────────────────────

#[test]
fn test_is_sse_request_with_event_stream_accept_returns_true_happy() {
    let mut headers = axum::http::HeaderMap::new();
    headers.insert(
        axum::http::header::ACCEPT,
        axum::http::HeaderValue::from_static("text/event-stream"),
    );
    assert!(AxumHttpServerHelper::is_sse_request(&headers));
}

#[test]
fn test_is_sse_request_with_json_accept_returns_false_error() {
    let mut headers = axum::http::HeaderMap::new();
    headers.insert(
        axum::http::header::ACCEPT,
        axum::http::HeaderValue::from_static("application/json"),
    );
    assert!(!AxumHttpServerHelper::is_sse_request(&headers));
}

#[test]
fn test_is_sse_request_with_no_accept_returns_false_edge() {
    let headers = axum::http::HeaderMap::new();
    assert!(!AxumHttpServerHelper::is_sse_request(&headers));
}

// ── collect_headers ───────────────────────────────────────────────────────────

#[test]
fn test_collect_headers_returns_content_type_happy() {
    let mut headers = axum::http::HeaderMap::new();
    headers.insert(
        axum::http::header::CONTENT_TYPE,
        axum::http::HeaderValue::from_static("application/json"),
    );
    let map = AxumHttpServerHelper::collect_headers(&headers);
    assert_eq!(
        map.get("content-type").map(|s| s.as_str()),
        Some("application/json")
    );
}

#[test]
fn test_collect_headers_empty_map_returns_empty_error() {
    let headers = axum::http::HeaderMap::new();
    let map = AxumHttpServerHelper::collect_headers(&headers);
    assert!(map.is_empty());
}

#[test]
fn test_collect_headers_keys_are_lowercase_edge() {
    let mut headers = axum::http::HeaderMap::new();
    headers.insert(
        axum::http::header::CONTENT_TYPE,
        axum::http::HeaderValue::from_static("text/plain"),
    );
    let map = AxumHttpServerHelper::collect_headers(&headers);
    for key in map.keys() {
        assert_eq!(
            key.to_lowercase(),
            *key,
            "header key must be lowercase: {key}"
        );
    }
}

// ── payload_too_large ─────────────────────────────────────────────────────────

#[test]
fn test_payload_too_large_returns_413_happy() {
    let resp = AxumHttpServerHelper::payload_too_large();
    assert_eq!(resp.status(), axum::http::StatusCode::PAYLOAD_TOO_LARGE);
}

#[test]
fn test_payload_too_large_content_type_is_plain_text_error() {
    let resp = AxumHttpServerHelper::payload_too_large();
    let ct = resp
        .headers()
        .get(axum::http::header::CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");
    assert!(
        ct.contains("text/plain"),
        "payload_too_large must set text/plain content-type; got: {ct}"
    );
}

#[test]
fn test_payload_too_large_called_twice_is_idempotent_edge() {
    let r1 = AxumHttpServerHelper::payload_too_large();
    let r2 = AxumHttpServerHelper::payload_too_large();
    assert_eq!(r1.status(), r2.status());
}

// ── internal_server_error ────────────────────────────────────────────────────

#[test]
fn test_internal_server_error_returns_500_happy() {
    let resp = AxumHttpServerHelper::internal_server_error("failure");
    assert_eq!(resp.status(), axum::http::StatusCode::INTERNAL_SERVER_ERROR);
}

#[test]
fn test_internal_server_error_returns_500_error() {
    let resp = AxumHttpServerHelper::internal_server_error("test failure");
    assert_eq!(resp.status(), axum::http::StatusCode::INTERNAL_SERVER_ERROR);
}

#[test]
fn test_internal_server_error_content_type_is_plain_text_edge() {
    let resp = AxumHttpServerHelper::internal_server_error("oops");
    let ct = resp
        .headers()
        .get(axum::http::header::CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");
    assert!(
        ct.contains("text/plain"),
        "internal_server_error must set text/plain content-type; got: {ct}"
    );
}

#[test]
fn test_helper_is_unit_struct_edge() {
    // AxumHttpServerHelper is a unit struct — construction is always valid.
    let h = AxumHttpServerHelper;
    assert_eq!(
        std::mem::size_of_val(&h),
        0,
        "AxumHttpServerHelper must be a zero-size type"
    );
}
