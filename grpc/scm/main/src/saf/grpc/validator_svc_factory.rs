//! SAF surface for ValidatorSvc — validator factory methods.
use crate::api::{GrpcValidationError, NoopGrpcValidator, Validator, ValidatorSvc};

/// Service identifier for the validator factory.
pub const VALIDATOR_SVC: &str = "validator";

impl ValidatorSvc {
    /// Validate a value using the provided [`Validator`].
    pub fn validate(v: &dyn Validator) -> Result<(), GrpcValidationError> {
        v.validate()
    }

    /// Create a no-op validator that always passes.
    pub fn create_noop() -> NoopGrpcValidator {
        NoopGrpcValidator
    }
}
