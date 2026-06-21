//! Tests for `HttpRequestBuilder` — fluent request builder.
// @covers HttpRequestBuilder
#![allow(clippy::unwrap_used)]

use swe_edge_runtime_http::{HttpMethod, HttpRequestBuilder};

#[test]
fn test_http_request_builder_get_builds_happy() {
    let req = HttpRequestBuilder::get("/").build();
    assert_eq!(req.method, HttpMethod::Get);
    assert_eq!(req.url, "/");
}

#[test]
fn test_http_request_builder_post_builds_happy() {
    let req = HttpRequestBuilder::post("/users").build();
    assert_eq!(req.method, HttpMethod::Post);
    assert_eq!(req.url, "/users");
}

#[test]
fn test_http_request_builder_with_header_error() {
    // "Error" path: header key typo — documents that headers are pass-through.
    let req = HttpRequestBuilder::get("/")
        .with_header("", "value")
        .build();
    // Even empty header names are stored (no validation in builder).
    assert!(req.headers.contains_key(""));
}

#[test]
fn test_http_request_builder_with_query_edge() {
    let req = HttpRequestBuilder::get("/search")
        .with_query("q", "")
        .build();
    assert_eq!(req.query.get("q").map(String::as_str), Some(""));
}

#[test]
fn test_http_request_builder_chaining_edge() {
    use swe_edge_runtime_http::HttpBody;
    let req = HttpRequestBuilder::post("/data")
        .with_header("Content-Type", "application/json")
        .with_query("v", "2")
        .with_body(HttpBody::Raw(b"{}".to_vec()))
        .build();
    assert_eq!(req.method, HttpMethod::Post);
    assert!(req.body.is_some());
}
