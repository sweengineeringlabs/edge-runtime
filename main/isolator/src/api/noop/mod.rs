//! `NoopIsolator` api interface — no-op isolation profile contract.

pub mod noop_isolation_profile;
#[expect(
    unused_imports,
    reason = "SEA api/ anchor — NoopIsolationProfile trait exported for consumers, not used internally"
)]
pub use noop_isolation_profile::NoopIsolationProfile;
