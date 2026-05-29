//! SAF layer — public factory surface for isolation profiles.

mod isolator_svc;

pub use crate::api::{
    Error,
    IsolationProfileRegistry,
    IsolatorConfig,
    IsolatorSvc,
    ProfileSpec,
    SweEdgeRuntimeIsolator,
    SweEdgeRuntimeIsolatorFactory,
    DefaultSweEdgeRuntimeIsolatorImpl,
    DefaultSweEdgeRuntimeIsolatorValidatorImpl,
    Validator,
};
