//! SAF layer — public factory surface for isolation profiles.

mod isolator_svc;

pub use crate::api::{
    DefaultSweEdgeRuntimeIsolatorImpl, DefaultSweEdgeRuntimeIsolatorValidatorImpl, Error,
    IsolationProfileRegistry, IsolatorConfig, IsolatorSvc, ProfileSpec, SweEdgeRuntimeIsolator,
    SweEdgeRuntimeIsolatorFactory, Validator,
};
