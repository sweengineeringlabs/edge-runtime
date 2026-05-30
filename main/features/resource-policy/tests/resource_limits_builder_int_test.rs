//! Integration tests for ResourceLimitsBuilder.

use swe_edge_runtime_resource_policy::{ResourceLimits, ResourceLimitsBuilder};

/// @covers: ResourceLimitsBuilder::new
#[test]
fn test_new_creates_empty_builder() {
    let limits = ResourceLimitsBuilder::new().build();
    assert!(limits.is_empty());
}

/// @covers: ResourceLimitsBuilder::timeout_ms
#[test]
fn test_timeout_ms_sets_timeout_field() {
    let limits = ResourceLimitsBuilder::new().timeout_ms(5_000).build();
    assert_eq!(limits.timeout_ms, Some(5_000));
    assert!(limits.output_bytes_cap.is_none());
}

/// @covers: ResourceLimitsBuilder::output_bytes_cap
#[test]
fn test_output_bytes_cap_sets_cap_field() {
    let limits = ResourceLimitsBuilder::new()
        .output_bytes_cap(1_048_576)
        .build();
    assert_eq!(limits.output_bytes_cap, Some(1_048_576));
    assert!(limits.timeout_ms.is_none());
}

/// @covers: ResourceLimitsBuilder::cpu_time_ms
#[test]
fn test_cpu_time_ms_sets_cpu_field() {
    let limits = ResourceLimitsBuilder::new().cpu_time_ms(0).build();
    assert_eq!(limits.cpu_time_ms, Some(0));
    assert!(!limits.is_empty());
}

/// @covers: ResourceLimitsBuilder::memory_bytes
#[test]
fn test_memory_bytes_sets_memory_field() {
    let limits = ResourceLimitsBuilder::new().memory_bytes(524_288).build();
    assert_eq!(limits.memory_bytes, Some(524_288));
}

/// @covers: ResourceLimitsBuilder::build
#[test]
fn test_build_returns_resource_limits_with_all_fields() {
    let limits = ResourceLimitsBuilder::new()
        .timeout_ms(1_000)
        .output_bytes_cap(2_000)
        .cpu_time_ms(3_000)
        .memory_bytes(4_000)
        .build();
    assert_eq!(limits.timeout_ms, Some(1_000));
    assert_eq!(limits.output_bytes_cap, Some(2_000));
    assert_eq!(limits.cpu_time_ms, Some(3_000));
    assert_eq!(limits.memory_bytes, Some(4_000));
    assert!(!limits.is_empty());
}

/// @covers: ResourceLimitsBuilder::build
#[test]
fn test_build_default_returns_all_none() {
    let limits: ResourceLimits = ResourceLimitsBuilder::new().build();
    assert!(limits.is_empty());
}
