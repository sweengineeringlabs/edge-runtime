//! `NoopRuntimeIsolator` — concrete no-op implementation of `SweEdgeRuntimeIsolator` and `Validator`.

use crate::api::error::Error;
use crate::api::traits::{SweEdgeRuntimeIsolator, Validator};

/// Concrete no-op runtime isolator.
///
/// Returned by [`IsolatorSvc::service`], [`IsolatorSvc::validator`],
/// [`SweEdgeRuntimeIsolatorFactory::create_swe_edge_runtime_isolator`], and
/// [`SweEdgeRuntimeIsolatorFactory::create_validator`] when no production
/// isolation policy is required (dev, CI, or environments without OS-level
/// subprocess restrictions).
#[derive(Debug, Default)]
pub struct NoopRuntimeIsolator;

impl SweEdgeRuntimeIsolator for NoopRuntimeIsolator {
    fn execute(&self) -> Result<(), Error> {
        Ok(())
    }
}

impl Validator for NoopRuntimeIsolator {
    fn validate(&self) -> Result<(), Error> {
        Ok(())
    }
}
