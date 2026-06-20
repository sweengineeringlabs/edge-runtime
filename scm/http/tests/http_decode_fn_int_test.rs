//! Tests for `HttpDecodeFn` — decode function type alias.
// @covers HttpDecodeFn
#![allow(clippy::unwrap_used)]

use swe_edge_runtime_http::{HttpDecodeFn, HttpIngressError, HttpRequest};

fn parse_url(req: &HttpRequest) -> Result<String, HttpIngressError> {
    Ok(req.url.clone())
}

fn fail_decode(_req: &HttpRequest) -> Result<u32, HttpIngressError> {
    Err(HttpIngressError::InvalidInput("not a number".to_string()))
}

#[test]
fn test_http_decode_fn_ok_happy() {
    let decode: HttpDecodeFn<String> = parse_url;
    let req = HttpRequest::get("/hello");
    assert_eq!(decode(&req).unwrap(), "/hello");
}

#[test]
fn test_http_decode_fn_err_error() {
    let decode: HttpDecodeFn<u32> = fail_decode;
    let req = HttpRequest::get("/");
    let result = decode(&req);
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        HttpIngressError::InvalidInput(_)
    ));
}

#[test]
fn test_http_decode_fn_empty_url_edge() {
    // Edge: decode from a request with an unusual URL.
    let decode: HttpDecodeFn<String> = parse_url;
    let req = HttpRequest::get("");
    let result = decode(&req).unwrap();
    assert!(result.is_empty());
}
