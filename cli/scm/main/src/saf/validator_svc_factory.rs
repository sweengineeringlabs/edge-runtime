//! Service factory for [`Validator`] — SAF implementation surface.

pub(crate) use crate::api::NoopValidator;

impl NoopValidator {
    /// Create a new [`NoopValidator`] that always returns `Ok(())`.
    pub fn create() -> Self {
        Self
    }

    /// Validate this target; returns `Ok(())` unconditionally.
    pub fn run_validate(&self) -> Result<(), crate::api::CliError> {
        use crate::api::Validator;
        self.validate()
    }
}
