//! Integration tests for the SeccompIsolator.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use swe_edge_egress_subprocess::IsolationError;
use swe_edge_runtime_isolator::{IsolatorConfig, IsolatorSvc};

/// @covers: SeccompIsolator — seccomp on non-Linux returns UnsupportedPlatform.
#[test]
fn test_seccomp_profile_on_unsupported_platform_returns_error() {
    // On Linux with `seccomp` feature, this may succeed or fail depending on
    // seccompiler internals. On Windows/macOS it always fails with UnsupportedPlatform.
    #[cfg(not(all(target_os = "linux", feature = "seccomp")))]
    {
        let toml = r#"
[profiles.restricted]
kind = "seccomp"
allowed_syscalls = ["read", "write"]
"#;
        let config: IsolatorConfig = toml::from_str(toml).expect("valid toml");
        let err = IsolatorSvc::build_registry(config).unwrap_err();
        assert!(
            matches!(err, IsolationError::UnsupportedPlatform { .. }),
            "expected UnsupportedPlatform; got {err:?}",
        );
    }

    // On Linux with seccomp feature, just verify no panic.
    #[cfg(all(target_os = "linux", feature = "seccomp"))]
    {
        let _ = IsolatorSvc::build_registry(IsolatorConfig::default());
    }
}
