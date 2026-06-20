//! Integration tests for [`swe_edge_runtime_grpc::GrpcIngressResult`].
#![allow(clippy::unwrap_used)]

use swe_edge_runtime_grpc::{GrpcIngressError, GrpcIngressResult};

/// @covers: GrpcIngressResult — Ok variant wraps a value
#[test]
fn test_ok_variant_holds_value_happy() {
    let result: GrpcIngressResult<u32> = Ok(42);
    assert!(result.is_ok());
    if let Ok(v) = result {
        assert_eq!(v, 42);
    }
}

/// @covers: GrpcIngressResult — Err variant holds GrpcIngressError
#[test]
fn test_err_variant_holds_error_error() {
    let result: GrpcIngressResult<()> = Err(GrpcIngressError::Internal("boom".into()));
    assert!(result.is_err());
}

/// @covers: GrpcIngressResult — Ok with unit type
#[test]
fn test_ok_unit_is_valid_edge() {
    let result: GrpcIngressResult<()> = Ok(());
    assert!(result.is_ok());
}
