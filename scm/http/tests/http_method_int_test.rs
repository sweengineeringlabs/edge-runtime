//! Tests for `HttpMethod` — HTTP method enum.
// @covers HttpMethod
use swe_edge_runtime_http::HttpMethod;

#[test]
fn test_http_method_default_is_get_happy() {
    let method = HttpMethod::default();
    assert_eq!(method, HttpMethod::Get);
}

#[test]
fn test_http_method_display_happy() {
    assert_eq!(HttpMethod::Post.to_string(), "POST");
    assert_eq!(HttpMethod::Get.to_string(), "GET");
    assert_eq!(HttpMethod::Delete.to_string(), "DELETE");
}

#[test]
fn test_http_method_all_variants_display_edge() {
    // Edge: verify no variant panics in Display.
    let methods = [
        HttpMethod::Get,
        HttpMethod::Post,
        HttpMethod::Put,
        HttpMethod::Patch,
        HttpMethod::Delete,
        HttpMethod::Head,
        HttpMethod::Options,
    ];
    for m in &methods {
        assert!(!m.to_string().is_empty());
    }
}

#[test]
fn test_http_method_equality_error() {
    // "Error" path: two different methods are not equal.
    assert_ne!(HttpMethod::Get, HttpMethod::Post);
}
