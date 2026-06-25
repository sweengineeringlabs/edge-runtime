//! SAF factory surface for Validator.

use crate::api::{NoopGrpcValidator, Validator, ValidatorSvc};

impl ValidatorSvc {
    /// Validate a value using the provided [`Validator`].
    pub fn validate(v: &dyn Validator) -> Result<(), String> {
        v.validate()
    }

    /// Create a no-op validator that always passes.
    pub fn create_noop() -> NoopGrpcValidator {
        NoopGrpcValidator
    }
}
