//! Tests for `HttpIngressError` variants and their display formatting.
// @covers HttpIngressError
#![allow(clippy::unwrap_used)]

use swe_edge_runtime_http::HttpIngressError;

#[test]
fn test_http_ingress_error_internal_display_happy() {
    let e = HttpIngressError::Internal("boom".to_string());
    assert!(e.to_string().contains("boom"));
}

#[test]
fn test_http_ingress_error_not_found_display_happy() {
    let e = HttpIngressError::NotFound("/missing".to_string());
    assert!(e.to_string().contains("/missing"));
}

#[test]
fn test_http_ingress_error_invalid_input_display_happy() {
    let e = HttpIngressError::InvalidInput("bad field".to_string());
    assert!(e.to_string().contains("bad field"));
}

#[test]
fn test_http_ingress_error_unauthorized_display_error() {
    let e = HttpIngressError::Unauthorized("no token".to_string());
    let s = e.to_string();
    assert!(s.contains("no token"));
}

#[test]
fn test_http_ingress_error_method_not_allowed_display_error() {
    let e = HttpIngressError::MethodNotAllowed("TRACE".to_string());
    assert!(e.to_string().contains("TRACE"));
}

#[test]
fn test_http_ingress_error_timeout_display_edge() {
    let e = HttpIngressError::Timeout("30s".to_string());
    assert!(e.to_string().contains("30s"));
}

#[test]
fn test_http_ingress_error_empty_message_edge() {
    // Edge: empty message string — display still works without panic.
    let e = HttpIngressError::Internal(String::new());
    let s = e.to_string();
    assert!(s.contains("internal"));
}
