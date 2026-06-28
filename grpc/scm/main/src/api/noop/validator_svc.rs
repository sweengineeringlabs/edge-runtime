//! ValidatorSvc type declaration and inherent impl.

use crate::api::{GrpcValidationError, NoopGrpcValidator, Validator};

/// Factory for validator objects.
pub struct ValidatorSvc;

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
