//! Tests for `HttpResponse` — HTTP response type.
// @covers HttpResponse
#![allow(clippy::unwrap_used)]

use swe_edge_runtime_http::HttpResponse;

#[test]
fn test_http_response_200_is_success_happy() {
    let resp = HttpResponse::new(200, b"ok".to_vec());
    assert!(resp.is_success());
    assert!(!resp.is_client_error());
    assert!(!resp.is_server_error());
}

#[test]
fn test_http_response_404_is_client_error_error() {
    let resp = HttpResponse::new(404, b"not found".to_vec());
    assert!(resp.is_client_error());
    assert!(!resp.is_success());
}

#[test]
fn test_http_response_500_is_server_error_error() {
    let resp = HttpResponse::new(500, b"oops".to_vec());
    assert!(resp.is_server_error());
    assert!(!resp.is_success());
}

#[test]
fn test_http_response_json_deserialises_happy() {
    let body = serde_json::to_vec(&serde_json::json!({"id": 1})).unwrap();
    let resp = HttpResponse::new(200, body);
    let val: serde_json::Value = resp.json().unwrap();
    assert_eq!(val["id"], 1);
}

#[test]
fn test_http_response_text_decodes_happy() {
    let resp = HttpResponse::new(200, b"hello world".to_vec());
    assert_eq!(resp.text().unwrap(), "hello world");
}

#[test]
fn test_http_response_text_invalid_utf8_error() {
    let resp = HttpResponse::new(200, vec![0xFF, 0xFE]);
    assert!(resp.text().is_err());
}

#[test]
fn test_http_response_header_lookup_case_insensitive_edge() {
    let mut resp = HttpResponse::new(200, vec![]);
    resp.headers
        .insert("Content-Type".to_string(), "application/json".to_string());
    assert_eq!(resp.header("content-type"), Some("application/json"));
}

#[test]
fn test_http_response_empty_body_edge() {
    let resp = HttpResponse::new(204, vec![]);
    assert!(resp.body.is_empty());
}
