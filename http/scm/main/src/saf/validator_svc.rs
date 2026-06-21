//! Service factory for [`Validator`] — SAF re-export surface.

pub use swe_edge_ingress_http::Validator;

pub use crate::api::noop::NoopValidator;

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
    pub fn run_validate(&self) -> Result<(), String> {
        use swe_edge_ingress_http::Validator;
        self.validate()
    }
}
