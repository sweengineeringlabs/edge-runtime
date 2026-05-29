//! Integration tests for the NoopIsolationProfile trait.

use swe_edge_egress_subprocess::IsolationProfile as _;
use swe_edge_runtime_isolator::IsolatorSvc;

/// @covers: NoopIsolationProfile
#[test]
fn test_noop_isolator_implements_isolation_profile() {
    let isolator = IsolatorSvc::create_noop_isolator();
    // The noop isolator's name is "noop".
    assert_eq!(isolator.name(), "noop");
}

/// @covers: NoopIsolationProfile
#[test]
fn test_noop_isolator_configure_returns_ok() {
    let mut cmd = tokio::process::Command::new("echo");
    let isolator = IsolatorSvc::create_noop_isolator();
    assert!(isolator.configure(&mut cmd).is_ok());
}
