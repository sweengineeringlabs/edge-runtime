//! `NoopIsolator` api interface marker — no-op isolation profile contract.

/// Marker trait for the no-op isolation profile.
///
/// Implemented by `core::noop::NoopIsolator` for dev and CI environments.
#[expect(dead_code, reason = "SEA api/ anchor — exported for consumers, not used internally")]
pub trait NoopIsolationProfile {}
