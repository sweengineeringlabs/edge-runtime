//! Integration tests for NoopGrpcValidator.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use swe_edge_runtime_grpc::NoopGrpcValidator;
use swe_edge_runtime_grpc::Validator;

#[test]
fn test_noop_grpc_validator_validate_always_returns_ok() {
    let v = NoopGrpcValidator;
    assert!(v.validate().is_ok());
}
