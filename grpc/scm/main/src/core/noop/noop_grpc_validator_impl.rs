//! No-op validator implementation.

use crate::api::{NoopGrpcValidator, Validator};

impl Validator for NoopGrpcValidator {
    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
