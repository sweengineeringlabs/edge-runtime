//! No-op [`Validator`] implementation.

use crate::api::{HttpIngressError, NoopValidator, Validator};

impl Validator for NoopValidator {
    fn validate(&self) -> Result<(), HttpIngressError> {
        Ok(())
    }
}
