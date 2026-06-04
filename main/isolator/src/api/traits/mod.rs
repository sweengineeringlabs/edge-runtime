//! Public trait definitions for swe_edge_runtime_isolator.

pub mod noop_isolation_profile;
pub mod seccomp_isolation_profile;
pub mod swe_edge_runtime_isolator;
pub mod validator;

#[expect(
    unused_imports,
    reason = "SEA api/ anchor — NoopIsolationProfile trait exported for consumers, not used internally"
)]
pub use noop_isolation_profile::NoopIsolationProfile;
#[expect(
    unused_imports,
    reason = "SEA api/ anchor — SeccompIsolationProfile trait exported for consumers, not used internally"
)]
pub use seccomp_isolation_profile::SeccompIsolationProfile;
pub use swe_edge_runtime_isolator::SweEdgeRuntimeIsolator;
pub use validator::Validator;
