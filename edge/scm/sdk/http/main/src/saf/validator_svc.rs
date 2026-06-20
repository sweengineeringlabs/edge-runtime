//! Service factory for [`Validator`] — SAF re-export surface.

pub use crate::api::{NoopValidator, Validator};

impl NoopValidator {
    /// Create a new [`NoopValidator`] that always returns `Ok(())`.
    ///
    /// Suitable for tests and composition roots that do not need real validation logic.
    pub fn create() -> Self {
        Self
    }

    /// Validate this target; returns `Ok(())` unconditionally.
    ///
    /// This is the canonical SAF entry point for callers that want to validate
    /// without naming the concrete implementor.
    pub fn run_validate(&self) -> Result<(), crate::api::HttpIngressError> {
        use crate::api::Validator;
        self.validate()
    }
}
