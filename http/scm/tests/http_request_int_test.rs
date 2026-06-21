//! Tests for `HttpRequest` — HTTP request type.
// @covers HttpRequest
#![allow(clippy::unwrap_used)]

use std::time::Duration;
use swe_edge_runtime_http::{HttpMethod, HttpRequest};

#[test]
fn test_http_request_get_constructs_happy() {
    let req = HttpRequest::get("/health");
    assert_eq!(req.method, HttpMethod::Get);
    assert_eq!(req.url, "/health");
    assert!(req.body.is_none());
}

#[test]
fn test_http_request_post_with_json_happy() {
    let req = HttpRequest::post("/users")
        .with_json(&serde_json::json!({"name": "alice"}))
        .unwrap();
    assert_eq!(req.method, HttpMethod::Post);
    assert!(req.body.is_some());
    assert_eq!(req.header("Content-Type"), Some("application/json"));
}

#[test]
fn test_http_request_with_header_case_insensitive_happy() {
    let req = HttpRequest::get("/").with_header("X-Trace-Id", "abc");
    assert_eq!(req.header("x-trace-id"), Some("abc"));
}

#[test]
fn test_http_request_missing_header_returns_none_error() {
    let req = HttpRequest::get("/");
    assert!(req.header("Authorization").is_none());
}

#[test]
fn test_http_request_delete_with_timeout_edge() {
    let req = HttpRequest::delete("/items/1").with_timeout(Duration::from_secs(5));
    assert_eq!(req.method, HttpMethod::Delete);
    assert_eq!(req.timeout, Some(Duration::from_secs(5)));
}

#[test]
fn test_http_request_put_constructs_happy() {
    let req = HttpRequest::put("/resource/42");
    assert_eq!(req.method, HttpMethod::Put);
}

#[test]
fn test_http_request_with_query_params_happy() {
    let req = HttpRequest::get("/search").with_query("q", "rust");
    assert_eq!(req.query.get("q").map(String::as_str), Some("rust"));
}

#[test]
fn test_http_request_with_form_body_edge() {
    let mut form = std::collections::HashMap::new();
    form.insert("field".to_string(), "val".to_string());
    let req = HttpRequest::post("/form").with_form(form);
    assert!(req.body.is_some());
    assert_eq!(
        req.header("Content-Type"),
        Some("application/x-www-form-urlencoded")
    );
}
