//! Default SweEdgeRuntimeIsolator implementation.

use crate::api::error::Error;
use crate::api::traits::SweEdgeRuntimeIsolator;
use crate::api::traits::Validator;

/// Default implementation of the SweEdgeRuntimeIsolator trait.
#[derive(Debug, Default)]
pub(crate) struct DefaultSweEdgeRuntimeIsolator;

impl DefaultSweEdgeRuntimeIsolator {
    /// Create a new default instance.
    #[cfg_attr(
        not(test),
        expect(
            dead_code,
            reason = "SEA core/ anchor — wired up when SAF factory is integrated"
        )
    )]
    pub(crate) fn new() -> Self {
        Self
    }
}

impl SweEdgeRuntimeIsolator for DefaultSweEdgeRuntimeIsolator {
    fn execute(&self) -> Result<(), Error> {
        tracing::debug!("executing swe_edge_runtime_isolator");
        Ok(())
    }
}

impl Validator for DefaultSweEdgeRuntimeIsolator {
    fn validate(&self) -> Result<(), Error> {
        tracing::debug!("validating swe_edge_runtime_isolator");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_creates_default_swe_edge_runtime_isolator() {
        let _svc = DefaultSweEdgeRuntimeIsolator::new();
    }

    #[test]
    fn test_execute_succeeds() {
        let svc = DefaultSweEdgeRuntimeIsolator::new();
        assert!(svc.execute().is_ok());
    }

    #[test]
    fn test_validate_succeeds() {
        let svc = DefaultSweEdgeRuntimeIsolator::new();
        assert!(svc.validate().is_ok());
    }
}
