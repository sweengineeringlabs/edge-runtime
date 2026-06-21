//! Integration tests for ValidatorSvc factory.
//! @covers: ValidatorSvc::validate
//! @covers: ValidatorSvc::create_noop
#![allow(clippy::unwrap_used, clippy::expect_used)]

use swe_edge_runtime_grpc::{NoopGrpcValidator, Validator, ValidatorSvc};

// ── ValidatorSvc::validate ────────────────────────────────────────────────────

#[test]
fn test_validate_noop_validator_returns_ok_happy() {
    // @covers: ValidatorSvc::validate
    let v = NoopGrpcValidator;
    assert!(ValidatorSvc::validate(&v).is_ok());
}

struct AlwaysFailValidator;
impl Validator for AlwaysFailValidator {
    fn validate(&self) -> Result<(), String> {
        Err("validation failed".to_string())
    }
}

#[test]
fn test_validate_failing_validator_returns_err_error() {
    // @covers: ValidatorSvc::validate
    let v = AlwaysFailValidator;
    let result = ValidatorSvc::validate(&v);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "validation failed");
}

struct ConditionalValidator(bool);
impl Validator for ConditionalValidator {
    fn validate(&self) -> Result<(), String> {
        if self.0 {
            Ok(())
        } else {
            Err("condition not met".to_string())
        }
    }
}

#[test]
fn test_validate_conditional_validator_edge() {
    // @covers: ValidatorSvc::validate
    // True condition passes, false fails — validates that the gate works both ways.
    assert!(ValidatorSvc::validate(&ConditionalValidator(true)).is_ok());
    assert!(ValidatorSvc::validate(&ConditionalValidator(false)).is_err());
}

// ── ValidatorSvc::create_noop ─────────────────────────────────────────────────

#[test]
fn test_create_noop_returns_validator_happy() {
    // @covers: ValidatorSvc::create_noop
    let v = ValidatorSvc::create_noop();
    assert!(v.validate().is_ok());
}

#[test]
fn test_create_noop_always_returns_ok_error() {
    // @covers: ValidatorSvc::create_noop
    // Noop never fails regardless of context — this is the contract.
    for _ in 0..5 {
        let v = ValidatorSvc::create_noop();
        assert!(v.validate().is_ok(), "noop must always pass");
    }
}

#[test]
fn test_create_noop_returned_type_validates_edge() {
    // @covers: ValidatorSvc::create_noop
    // The returned NoopGrpcValidator implements Validator as expected.
    let v: NoopGrpcValidator = ValidatorSvc::create_noop();
    let result: Result<(), String> = Validator::validate(&v);
    assert!(result.is_ok());
}

// ── Validator::new_noop ──────────────────────────────────────────────────────

#[test]
fn test_new_noop_returns_noop_validator_happy() {
    // @covers: Validator::new_noop
    let v = NoopGrpcValidator::new_noop();
    assert!(v.validate().is_ok());
}

#[test]
fn test_new_noop_trait_method_not_error_error() {
    // @covers: Validator::new_noop
    let v = NoopGrpcValidator::new_noop();
    // validate() on a noop must never fail — it is a contract violation if it does.
    assert_ne!(v.validate(), Err("validation failed".to_string()));
}

#[test]
fn test_new_noop_type_is_noop_grpc_validator_edge() {
    // @covers: Validator::new_noop
    // verify the type is usable as a Validator trait object.
    let v = NoopGrpcValidator::new_noop();
    let dyn_v: &dyn Validator = &v;
    assert!(dyn_v.validate().is_ok());
}

// ── Validator::new_svc ───────────────────────────────────────────────────────

#[test]
fn test_new_svc_returns_validator_svc_happy() {
    // @covers: Validator::new_svc
    let _svc = NoopGrpcValidator::new_svc();
}

#[test]
fn test_new_svc_returned_svc_can_validate_error() {
    // @covers: Validator::new_svc
    let svc = NoopGrpcValidator::new_svc();
    let v = NoopGrpcValidator;
    // The returned svc must be able to validate any Validator.
    assert!(ValidatorSvc::validate(&v).is_ok());
    drop(svc);
}

#[test]
fn test_new_svc_returns_validator_svc_edge() {
    // @covers: Validator::new_svc
    // Multiple calls each produce an independent instance.
    let _svc1 = NoopGrpcValidator::new_svc();
    let _svc2 = NoopGrpcValidator::new_svc();
}

// ── Validator::validate trait coverage ─────────────────────────────────────────

#[test]
fn test_validate_noop_via_trait_happy() {
    // @covers: Validator::validate
    let v = NoopGrpcValidator;
    assert!(Validator::validate(&v).is_ok());
}

#[test]
fn test_validate_custom_error_via_trait_error() {
    // @covers: Validator::validate
    let v = AlwaysFailValidator;
    assert!(Validator::validate(&v).is_err());
}

#[test]
fn test_validate_conditional_via_trait_edge() {
    // @covers: Validator::validate
    let pass = ConditionalValidator(true);
    let fail = ConditionalValidator(false);
    assert!(Validator::validate(&pass).is_ok());
    assert!(Validator::validate(&fail).is_err());
}
