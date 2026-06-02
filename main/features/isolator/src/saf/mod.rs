//! SAF layer — public factory surface for isolation profiles.

mod isolator_svc;

#[expect(
    unused_imports,
    reason = "SEA saf/ anchor — all items exported for consumers, not used internally"
)]
pub use crate::api::{
    Error, IsolationProfileRegistry, IsolatorConfig, IsolatorSvc, NoopIsolationProfile,
    NoopRuntimeIsolator, ProfileSpec, SweEdgeRuntimeIsolator, SweEdgeRuntimeIsolatorFactory,
    Validator,
};
