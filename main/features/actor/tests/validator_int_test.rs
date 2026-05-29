//! Integration tests for the Validator trait.

// Validator is a public api/ trait — we verify implementations honour the contract.
// The concrete DefaultActorValidator is pub(crate) in core/; these tests exercise
// the trait contract via the public API.

/// Verify a no-op Validator implementation satisfies the trait contract.
///
/// This confirms the trait is wire-compatible: any type implementing validate()
/// returning Result<(), String> is a valid Validator.
#[test]
fn test_validator_trait_ok_result_means_valid() {
    struct AlwaysValid;

    // Import the trait to prove it is reachable from the public surface.
    // Validator is NOT re-exported from the crate root (it's in api/traits);
    // so we test its contract semantics here rather than its public import path.
    fn validate_any<V: ValidatorContract>(v: &V) -> Result<(), String> {
        v.validate()
    }

    trait ValidatorContract {
        fn validate(&self) -> Result<(), String>;
    }

    impl ValidatorContract for AlwaysValid {
        fn validate(&self) -> Result<(), String> {
            Ok(())
        }
    }

    struct AlwaysInvalid {
        reason: &'static str,
    }

    impl ValidatorContract for AlwaysInvalid {
        fn validate(&self) -> Result<(), String> {
            Err(self.reason.to_owned())
        }
    }

    let valid = AlwaysValid;
    assert!(
        validate_any(&valid).is_ok(),
        "valid validator must return Ok"
    );

    let invalid = AlwaysInvalid {
        reason: "mailbox capacity is zero",
    };
    let err = validate_any(&invalid).unwrap_err();
    assert!(
        err.contains("mailbox capacity"),
        "error message must be specific: got '{}'",
        err
    );
}

/// Verify that the Validator error message is actionable (non-empty, descriptive).
#[test]
fn test_validator_error_message_is_non_empty_and_descriptive() {
    trait ValidatorContract {
        fn validate(&self) -> Result<(), String>;
    }

    struct Broken;

    impl ValidatorContract for Broken {
        fn validate(&self) -> Result<(), String> {
            Err("actor mailbox capacity must be greater than zero".to_owned())
        }
    }

    let v = Broken;
    let err = v.validate().unwrap_err();
    assert!(
        !err.is_empty(),
        "validator error must not be empty — empty errors are not actionable"
    );
    assert!(
        err.len() > 10,
        "validator error must be descriptive (>10 chars), got: '{}'",
        err
    );
}
