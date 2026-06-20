//! Integration tests for the [`Validator`] trait contract.
#![allow(clippy::unwrap_used)]

use swe_edge_runtime_grpc::{GrpcIngressError, NoopGrpcValidator, Validator};

/// @covers: Validator::validate — happy path via NoopGrpcValidator
#[test]
fn test_validate_noop_returns_ok_happy() {
    let v = NoopGrpcValidator;
    assert!(v.validate().is_ok());
}

/// @covers: Validator::validate — custom impl can return Err
#[test]
fn test_validate_always_fail_impl_returns_error() {
    struct AlwaysFail;
    impl Validator for AlwaysFail {
        fn validate(&self) -> Result<(), GrpcIngressError> {
            Err(GrpcIngressError::InvalidInput("forced".into()))
        }
    }
    assert!(AlwaysFail.validate().is_err());
}

/// @covers: Validator::validate — idempotent on NoopGrpcValidator
#[test]
fn test_validate_noop_idempotent_edge() {
    let v = NoopGrpcValidator;
    assert!(v.validate().is_ok());
    assert!(v.validate().is_ok());
}
