//! Default SweEdgeRuntimeResourcePolicy implementation.

use crate::api::error::Error;
use crate::api::swe_edge_runtime_resource_policy::SweEdgeRuntimeResourcePolicy;
use crate::api::traits::Validator;

/// Default implementation of the SweEdgeRuntimeResourcePolicy trait.
#[derive(Debug, Default)]
pub(crate) struct DefaultSweEdgeRuntimeResourcePolicy;

impl DefaultSweEdgeRuntimeResourcePolicy {
    /// Create a new default instance.
    pub(crate) fn new() -> Self {
        Self
    }
}

impl SweEdgeRuntimeResourcePolicy for DefaultSweEdgeRuntimeResourcePolicy {
    fn execute(&self) -> Result<(), Error> {
        tracing::debug!("executing swe_edge_runtime_resource_policy");
        Ok(())
    }
}

impl Validator for DefaultSweEdgeRuntimeResourcePolicy {
    fn validate(&self) -> Result<(), Error> {
        tracing::debug!("validating swe_edge_runtime_resource_policy");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_creates_default_swe_edge_runtime_resource_policy() {
        let _svc = DefaultSweEdgeRuntimeResourcePolicy::new();
    }

    #[test]
    fn test_execute_succeeds() {
        let svc = DefaultSweEdgeRuntimeResourcePolicy::new();
        assert!(svc.execute().is_ok());
    }

    #[test]
    fn test_validate_succeeds() {
        let svc = DefaultSweEdgeRuntimeResourcePolicy::new();
        assert!(svc.validate().is_ok());
    }
}
