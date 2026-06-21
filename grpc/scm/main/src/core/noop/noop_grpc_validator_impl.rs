//! No-op validator implementation.

/// Validation contract for gRPC server configurations and interceptors.
pub trait Validator {
    /// Returns `Ok(())` when the value is valid, or a human-readable error.
    fn validate(&self) -> Result<(), String>;
}

/// No-op validator — always passes.
pub struct NoopGrpcValidator;

impl Validator for NoopGrpcValidator {
    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

impl NoopGrpcValidator {
    /// Create a reference-counted no-op validator.
    pub fn create() -> std::sync::Arc<Self> {
        std::sync::Arc::new(Self)
    }
}
