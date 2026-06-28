//! Integration tests for the Validator trait.
//! @covers: Validator::validate
#![allow(clippy::unwrap_used, clippy::expect_used)]

use swe_edge_runtime_grpc::{GrpcValidationError, NoopGrpcValidator, Validator};

#[test]
fn test_validate_noop_returns_ok_happy() {
    // @covers: Validator::validate
    let v = NoopGrpcValidator;
    let result = v.validate();
    assert!(result.is_ok(), "noop validator must always pass");
    assert_ne!(
        result,
        Err(GrpcValidationError::Invalid("rejected".to_string())),
        "noop must not produce RejectValidator's message"
    );
}

struct RejectValidator;
impl Validator for RejectValidator {
    fn validate(&self) -> Result<(), GrpcValidationError> {
        Err(GrpcValidationError::Invalid("rejected".to_string()))
    }
}

#[test]
fn test_validate_custom_error_returns_err_error() {
    // @covers: Validator::validate
    let v = RejectValidator;
    let result = v.validate();
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err(),
        GrpcValidationError::Invalid("rejected".to_string())
    );
}

#[test]
fn test_validate_noop_as_trait_object_edge() {
    // @covers: Validator::validate
    let v = NoopGrpcValidator;
    let dyn_v: &dyn Validator = &v;
    assert!(
        dyn_v.validate().is_ok(),
        "noop via trait object must always pass"
    );
    let reject = RejectValidator;
    let dyn_reject: &dyn Validator = &reject;
    assert_eq!(
        dyn_reject.validate().unwrap_err(),
        GrpcValidationError::Invalid("rejected".to_string())
    );
}
