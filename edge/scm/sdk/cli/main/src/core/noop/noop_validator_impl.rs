//! No-op [`Validator`] implementation.

use crate::api::{CliError, NoopValidator, Validator};

impl Validator for NoopValidator {
    fn validate(&self) -> Result<(), CliError> {
        Ok(())
    }
}
