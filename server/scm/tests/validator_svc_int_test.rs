//! Integration tests for the validator_svc SAF surface.

use swe_edge_runtime::{RuntimeConfig, RuntimeError, Validator, VALIDATOR_SVC};

struct LenientValidator;
impl Validator for LenientValidator {
    type Target = RuntimeConfig;
    type Error = RuntimeError;
    fn validate(&self, _value: &RuntimeConfig) -> Result<(), RuntimeError> {
        Ok(())
    }
}

struct StrictValidator;
impl Validator for StrictValidator {
    type Target = RuntimeConfig;
    type Error = RuntimeError;
    fn validate(&self, value: &RuntimeConfig) -> Result<(), RuntimeError> {
        if value.http_bind.is_empty() {
            Err(RuntimeError::StartFailed(
                "http_bind must not be empty".into(),
            ))
        } else {
            Ok(())
        }
    }
}

/// @covers: VALIDATOR_SVC
#[test]
fn test_validator_svc_slug_is_correct_happy() {
    assert_eq!(VALIDATOR_SVC, "validator");
}

// ── Validator::validate ───────────────────────────────────────────────────────

#[test]
fn test_validate_lenient_validator_accepts_default_config_happy() {
    let cfg = RuntimeConfig::default();
    assert!(LenientValidator.validate(&cfg).is_ok());
}

#[test]
fn test_validate_strict_validator_rejects_empty_http_bind_error() {
    let cfg = RuntimeConfig {
        http_bind: String::new(),
        ..RuntimeConfig::default()
    };
    assert!(StrictValidator.validate(&cfg).is_err());
}

#[test]
fn test_validate_strict_validator_accepts_non_empty_bind_edge() {
    let cfg = RuntimeConfig {
        http_bind: "0.0.0.0:8080".into(),
        ..RuntimeConfig::default()
    };
    assert!(StrictValidator.validate(&cfg).is_ok());
}
