//! SAF layer — public factory surface for isolation profiles.

mod isolator_svc;

pub use crate::api::{
    Error, IsolationProfileRegistry, IsolatorConfig, IsolatorSvc, NoopIsolationProfile,
    NoopRuntimeIsolator, ProfileSpec, SweEdgeRuntimeIsolator, SweEdgeRuntimeIsolatorFactory,
    Validator,
};
