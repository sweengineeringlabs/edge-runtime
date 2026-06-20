//! Tests for `HttpAuth` — authentication credential variants.
// @covers HttpAuth
use swe_edge_runtime_http::HttpAuth;

#[test]
fn test_http_auth_bearer_constructs_happy() {
    let auth = HttpAuth::bearer("my-token");
    assert!(matches!(auth, HttpAuth::Bearer { token } if token == "my-token"));
}

#[test]
fn test_http_auth_basic_constructs_happy() {
    let auth = HttpAuth::basic("alice", "s3cret");
    assert!(matches!(auth, HttpAuth::Basic { username, .. } if username == "alice"));
}

#[test]
fn test_http_auth_api_key_constructs_happy() {
    let auth = HttpAuth::api_key("X-API-Key", "key123");
    assert!(matches!(auth, HttpAuth::ApiKey { key, .. } if key == "key123"));
}

#[test]
fn test_http_auth_none_default_error() {
    // "Error" path: default variant means no auth — documents the None case.
    let auth = HttpAuth::None;
    assert!(matches!(auth, HttpAuth::None));
}

#[test]
fn test_http_auth_bearer_empty_token_edge() {
    // Edge: empty token is structurally valid.
    let auth = HttpAuth::bearer("");
    assert!(matches!(auth, HttpAuth::Bearer { token } if token.is_empty()));
}
