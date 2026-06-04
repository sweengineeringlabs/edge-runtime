//! Integration tests for ResourceLimits.

use swe_edge_runtime_resource_policy::ResourceLimits;

/// @covers: ResourceLimits::is_empty
#[test]
fn test_resource_limits_default_is_empty() {
    assert!(ResourceLimits::default().is_empty());
}

/// @covers: ResourceLimits::is_empty
#[test]
fn test_resource_limits_with_timeout_is_not_empty() {
    let l = ResourceLimits {
        timeout_ms: Some(1_000),
        ..Default::default()
    };
    assert!(!l.is_empty());
}

/// @covers: ResourceLimits::is_empty
#[test]
fn test_resource_limits_zero_cpu_is_not_empty() {
    let l = ResourceLimits {
        cpu_time_ms: Some(0),
        ..Default::default()
    };
    assert!(!l.is_empty());
}
