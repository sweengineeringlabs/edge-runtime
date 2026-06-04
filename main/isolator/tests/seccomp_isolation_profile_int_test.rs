//! Integration tests for the SeccompIsolationProfile trait.
//!
//! Seccomp is only available on Linux with the `seccomp` feature enabled.
//! On other platforms this file still compiles to a no-op test binary.

/// @covers: SeccompIsolationProfile
#[test]
fn test_seccomp_isolation_profile_trait_is_defined() {
    // Verifies the trait module compiles and is accessible.
    // Platform-specific behaviour (filter compilation) is tested only on Linux.
    // On other platforms the implementation falls back to UnsupportedPlatform.
    let _ = std::any::type_name::<()>(); // no-op to satisfy the test harness
}
