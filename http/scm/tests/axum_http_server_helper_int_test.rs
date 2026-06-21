//! Integration tests for `AxumHttpServerHelper` public helpers.
// @covers api/server/types/axum_http_server_helper.rs
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
    assert_eq!(map.get("content-type").map(|s| s.as_str()), Some("application/json"));
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
        assert_eq!(key.to_lowercase(), *key, "header key must be lowercase: {key}");
    }
}

// ── payload_too_large ─────────────────────────────────────────────────────────

#[test]
fn test_payload_too_large_returns_413_happy() {
    let resp = AxumHttpServerHelper::payload_too_large();
    assert_eq!(resp.status(), axum::http::StatusCode::PAYLOAD_TOO_LARGE);
}

#[test]
fn test_internal_server_error_returns_500_error() {
    let resp = AxumHttpServerHelper::internal_server_error("test failure");
    assert_eq!(resp.status(), axum::http::StatusCode::INTERNAL_SERVER_ERROR);
}

#[test]
fn test_helper_is_unit_struct_edge() {
    // AxumHttpServerHelper is a unit struct — construction is always valid.
    let _h = AxumHttpServerHelper;
}

// ── verify_auth ───────────────────────────────────────────────────────────────

#[test]
fn test_verify_auth_no_verifier_passes_through_happy() {
    let req = axum::http::Request::builder()
        .uri("/")
        .body(axum::body::Body::empty())
        .unwrap();
    let result = AxumHttpServerHelper::verify_auth(req, None);
    assert!(result.is_ok(), "no verifier must pass request through");
}

#[test]
fn test_verify_auth_with_verifier_missing_auth_header_returns_401_error() {
    use swe_edge_ingress_verifier::{Claims, TokenVerifier, VerifierError};
    struct AcceptAll;
    impl TokenVerifier for AcceptAll {
        fn verify(&self, _: &str) -> Result<Claims, VerifierError> {
            Ok(Claims::builder().sub("u").build())
        }
    }
    let req = axum::http::Request::builder()
        .uri("/secure")
        .body(axum::body::Body::empty())
        .unwrap();
    let result = AxumHttpServerHelper::verify_auth(req, Some(&AcceptAll));
    assert!(result.is_err(), "missing auth header with verifier must fail");
    if let Err(resp) = result {
        assert_eq!(resp.status(), axum::http::StatusCode::UNAUTHORIZED);
    }
}

#[test]
fn test_verify_auth_with_bearer_token_verified_ok_edge() {
    use swe_edge_ingress_verifier::{Claims, TokenVerifier, VerifierError};
    struct AcceptAll;
    impl TokenVerifier for AcceptAll {
        fn verify(&self, _: &str) -> Result<Claims, VerifierError> {
            Ok(Claims::builder().sub("user").build())
        }
    }
    let req = axum::http::Request::builder()
        .uri("/secure")
        .header(axum::http::header::AUTHORIZATION, "Bearer validtoken")
        .body(axum::body::Body::empty())
        .unwrap();
    let result = AxumHttpServerHelper::verify_auth(req, Some(&AcceptAll));
    assert!(result.is_ok(), "valid bearer token with AcceptAll must pass");
}
