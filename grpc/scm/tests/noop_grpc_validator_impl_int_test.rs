//! Integration tests for NoopGrpcValidator.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use swe_edge_runtime_grpc::GrpcValidationError;
use swe_edge_runtime_grpc::NoopGrpcValidator;
use swe_edge_runtime_grpc::Validator;

#[test]
fn test_noop_grpc_validator_validate_always_returns_ok() {
    let v = NoopGrpcValidator;
    let result = v.validate();
    assert!(result.is_ok(), "noop validator must always return Ok");
    assert_ne!(
        result,
        Err(GrpcValidationError::Invalid("unexpected error".to_string())),
        "noop must not produce an error"
    );
}
