//! `NoopRuntimeResourcePolicy` — public concrete type returned by SAF factories.

use crate::api::error::Error;
use crate::api::traits::SweEdgeRuntimeResourcePolicy;
use crate::api::traits::Validator;

/// No-op implementation of the resource policy contract.
///
/// Returned by SAF factory functions so callers can name the concrete type.
/// Both [`SweEdgeRuntimeResourcePolicy::execute`] and [`Validator::validate`]
/// succeed immediately without performing any work.
#[derive(Debug, Default)]
pub struct NoopRuntimeResourcePolicy;

impl SweEdgeRuntimeResourcePolicy for NoopRuntimeResourcePolicy {
    fn execute(&self) -> Result<(), Error> {
        Ok(())
    }
}

impl Validator for NoopRuntimeResourcePolicy {
    fn validate(&self) -> Result<(), Error> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execute_noop_runtime_resource_policy_returns_ok() {
        let policy = NoopRuntimeResourcePolicy;
        assert!(policy.execute().is_ok());
    }

    #[test]
    fn test_validate_noop_runtime_resource_policy_returns_ok() {
        let policy = NoopRuntimeResourcePolicy;
        assert!(policy.validate().is_ok());
    }
}
