//! `SeccompIsolator` api interface marker — seccomp-bpf isolation profile contract.

/// Marker trait for the seccomp-bpf isolation profile.
///
/// Implemented by `core::seccomp::SeccompIsolator` on Linux when the
/// `seccomp` feature is enabled.
pub trait SeccompIsolationProfile {}
