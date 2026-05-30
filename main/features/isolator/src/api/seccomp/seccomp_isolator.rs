//! `SeccompIsolator` api interface contract.

/// Marker trait representing the seccomp-bpf isolator contract.
///
/// Implemented by `core::seccomp::SeccompIsolator` on Linux when the
/// `seccomp` feature is enabled.
pub trait SeccompIsolator {}
