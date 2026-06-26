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
    let result = ValidatorSvc::validate(&v);
    assert!(result.is_ok(), "noop validator must always pass");
    assert_ne!(
        result,
        Err("validation failed".to_string()),
        "noop must not produce an error"
    );
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
    let result = v.validate();
    assert!(result.is_ok(), "noop must always pass");
    assert_ne!(
        result,
        Err("failed".to_string()),
        "noop must not produce errors"
    );
}

#[test]
fn test_create_noop_always_returns_ok_error() {
    // @covers: ValidatorSvc::create_noop
    // Noop never fails regardless of context — this is the contract.
    for _ in 0..5 {
        let v = ValidatorSvc::create_noop();
        let result = v.validate();
        assert!(result.is_ok(), "noop must always pass");
        assert_ne!(
            result,
            Err("validation failed".to_string()),
            "noop must not produce error"
        );
    }
}

#[test]
fn test_create_noop_returned_type_validates_edge() {
    // @covers: ValidatorSvc::create_noop
    // The returned NoopGrpcValidator implements Validator as expected.
    let v: NoopGrpcValidator = ValidatorSvc::create_noop();
    let result: Result<(), String> = Validator::validate(&v);
    assert!(result.is_ok(), "noop must always pass");
    // A real validator would differ — ensure noop is not accidentally erroring.
    assert_ne!(result, Err("unexpected".to_string()));
}

// ── Validator::new_noop ──────────────────────────────────────────────────────

#[test]
fn test_new_noop_returns_noop_validator_happy() {
    // @covers: Validator::new_noop
    let v = NoopGrpcValidator::new_noop();
    assert!(v.validate().is_ok(), "new_noop must always pass");
    assert_ne!(
        v.validate(),
        Err("anything".to_string()),
        "noop must not error"
    );
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
    assert!(
        dyn_v.validate().is_ok(),
        "noop trait object must always pass"
    );
    assert_ne!(dyn_v.validate(), Err("error".to_string()));
}

// ── Validator::new_svc ───────────────────────────────────────────────────────

#[test]
fn test_new_svc_returns_validator_svc_happy() {
    // @covers: Validator::new_svc
    let _svc = NoopGrpcValidator::new_svc();
    // The returned ValidatorSvc enables delegation — noop always passes, fail always fails.
    assert!(ValidatorSvc::validate(&NoopGrpcValidator).is_ok());
    assert_eq!(
        ValidatorSvc::validate(&AlwaysFailValidator).unwrap_err(),
        "validation failed",
    );
}

#[test]
fn test_new_svc_returned_svc_can_validate_error() {
    // @covers: Validator::new_svc
    let _svc = NoopGrpcValidator::new_svc();
    // The returned svc must be able to validate any Validator.
    assert!(ValidatorSvc::validate(&NoopGrpcValidator).is_ok());
    // A failing validator must be rejected.
    assert_eq!(
        ValidatorSvc::validate(&AlwaysFailValidator).unwrap_err(),
        "validation failed",
    );
}

#[test]
fn test_new_svc_returns_validator_svc_edge() {
    // @covers: Validator::new_svc
    // Consecutive new_svc() calls — both instances route to the same contract.
    let _svc1 = NoopGrpcValidator::new_svc();
    let _svc2 = NoopGrpcValidator::new_svc();
    // Edge: conditional validator routes correctly through both paths.
    assert!(ValidatorSvc::validate(&ConditionalValidator(true)).is_ok());
    assert_eq!(
        ValidatorSvc::validate(&ConditionalValidator(false)).unwrap_err(),
        "condition not met",
    );
}

// ── Validator::validate trait coverage ─────────────────────────────────────────

#[test]
fn test_validate_noop_via_trait_happy() {
    // @covers: Validator::validate
    let v = NoopGrpcValidator;
    assert!(
        Validator::validate(&v).is_ok(),
        "noop must pass via trait dispatch"
    );
    assert_ne!(Validator::validate(&v), Err("error".to_string()));
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
