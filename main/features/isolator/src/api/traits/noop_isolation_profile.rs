//! `NoopIsolator` api interface marker — no-op isolation profile contract.

/// Marker trait for the no-op isolation profile.
///
/// Implemented by `core::noop::NoopIsolator` for dev and CI environments.
pub trait NoopIsolationProfile {}
