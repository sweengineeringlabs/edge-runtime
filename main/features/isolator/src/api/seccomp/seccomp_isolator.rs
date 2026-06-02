//! `SeccompIsolator` api interface contract.

/// Marker trait representing the seccomp-bpf isolator contract.
///
/// Implemented by `core::seccomp::SeccompIsolator` on Linux when the
/// `seccomp` feature is enabled.
#[expect(dead_code, reason = "SEA api/ anchor — exported for consumers, not used internally")]
pub trait SeccompIsolator {}
