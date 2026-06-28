//! No-op validator implementation.

use crate::api::{GrpcValidationError, NoopGrpcValidator, Validator};

impl Validator for NoopGrpcValidator {
    fn validate(&self) -> Result<(), GrpcValidationError> {
        Ok(())
    }
}
