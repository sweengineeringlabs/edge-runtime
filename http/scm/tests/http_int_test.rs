//! Integration tests exercising the `http` crate dependency.
//!
//! The `http` crate provides `http::Request`, `http::Response`, and related
//! primitives used in the TLS serve path (hyper compatibility layer).
// @covers dep:http
#![allow(clippy::unwrap_used)]

use http::{Method, Request, Response, StatusCode, Uri, Version};

// ── Method ───────────────────────────────────────────────────────────────────

#[test]
fn test_http_method_get_is_get_happy() {
    assert_eq!(Method::GET, Method::GET);
    assert_eq!(Method::GET.as_str(), "GET");
}

#[test]
fn test_http_method_post_is_post_error() {
    assert_eq!(Method::POST.as_str(), "POST");
    assert_ne!(Method::GET, Method::POST);
}

#[test]
fn test_http_method_custom_from_str_edge() {
    use std::str::FromStr;
    let m = Method::from_str("PATCH").unwrap();
    assert_eq!(m.as_str(), "PATCH");
}

// ── StatusCode ────────────────────────────────────────────────────────────────

#[test]
fn test_http_status_code_200_is_success_happy() {
    let s = StatusCode::OK;
    assert!(s.is_success());
    assert_eq!(s.as_u16(), 200);
}

#[test]
fn test_http_status_code_404_is_client_error_error() {
    let s = StatusCode::NOT_FOUND;
    assert!(s.is_client_error());
    assert_eq!(s.as_u16(), 404);
}

#[test]
fn test_http_status_code_500_is_server_error_edge() {
    let s = StatusCode::INTERNAL_SERVER_ERROR;
    assert!(s.is_server_error());
}

// ── Uri ───────────────────────────────────────────────────────────────────────

#[test]
fn test_http_uri_path_only_parses_happy() {
    let uri: Uri = "/ping".parse().unwrap();
    assert_eq!(uri.path(), "/ping");
}

#[test]
fn test_http_uri_full_url_parses_error() {
    let uri: Uri = "http://localhost:8080/api".parse().unwrap();
    assert_eq!(uri.host(), Some("localhost"));
}

#[test]
fn test_http_uri_with_query_string_edge() {
    let uri: Uri = "/search?q=foo&page=1".parse().unwrap();
    assert_eq!(uri.path(), "/search");
    assert_eq!(uri.query(), Some("q=foo&page=1"));
}

// ── Request ───────────────────────────────────────────────────────────────────

#[test]
fn test_http_request_builder_constructs_happy() {
    let req = Request::builder()
        .method(Method::GET)
        .uri("/hello")
        .body(())
        .unwrap();
    assert_eq!(req.method(), Method::GET);
    assert_eq!(req.uri().path(), "/hello");
}

#[test]
fn test_http_request_post_with_body_error() {
    let req = Request::builder()
        .method(Method::POST)
        .uri("/echo")
        .body("hello".to_string())
        .unwrap();
    assert_eq!(req.method(), Method::POST);
    assert_eq!(req.body(), "hello");
}

#[test]
fn test_http_request_version_http11_edge() {
    let req = Request::builder()
        .version(Version::HTTP_11)
        .uri("/")
        .body(())
        .unwrap();
    assert_eq!(req.version(), Version::HTTP_11);
}

// ── Response ─────────────────────────────────────────────────────────────────

#[test]
fn test_http_response_builder_ok_happy() {
    let resp = Response::builder()
        .status(StatusCode::OK)
        .body("body".to_string())
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
}

#[test]
fn test_http_response_not_found_error() {
    let resp = Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(())
        .unwrap();
    assert!(resp.status().is_client_error());
}

#[test]
fn test_http_response_with_header_edge() {
    let resp = Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "application/json")
        .body(())
        .unwrap();
    assert!(resp.headers().contains_key("content-type"));
}
