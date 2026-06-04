//! `SeccompIsolator` api interface marker — seccomp-bpf isolation profile contract.

/// Marker trait for the seccomp-bpf isolation profile.
///
/// Implemented by `core::seccomp::SeccompIsolator` on Linux when the
/// `seccomp` feature is enabled.
#[expect(
    dead_code,
    reason = "SEA api/ anchor — exported for consumers, not used internally"
)]
pub trait SeccompIsolationProfile {}
