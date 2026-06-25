//! Tests for `HttpIngressResult` — result type alias for ingress operations.
// @covers HttpIngressResult
#![allow(clippy::unwrap_used)]

use swe_edge_runtime_http::{HttpIngressError, HttpIngressResult};

#[test]
fn test_http_ingress_result_ok_happy() {
    let result: HttpIngressResult<u32> = Ok(42);
    assert!(matches!(result, Ok(42)));
}

#[test]
fn test_http_ingress_result_err_error() {
    let result: HttpIngressResult<u32> = Err(HttpIngressError::Internal("oops".to_string()));
    assert!(result.is_err());
}

#[test]
fn test_http_ingress_result_ok_unit_edge() {
    // Edge: unit Ok — used when the operation has no return value.
    let result: HttpIngressResult<()> = Ok(());
    assert!(
        matches!(result, Ok(())),
        "unit Ok must match Ok(()) exactly"
    );
}
