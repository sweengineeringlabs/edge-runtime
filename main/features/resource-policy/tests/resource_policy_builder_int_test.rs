//! Integration tests for ResourcePolicyBuilder.

use swe_edge_runtime_resource_policy::ResourcePolicyBuilder;

/// @covers: ResourcePolicyBuilder::new
#[test]
fn test_new_creates_zero_value_builder() {
    let policy = ResourcePolicyBuilder::new().build();
    assert_eq!(policy.timeout_ms, 0);
    assert!(policy.name.is_empty());
}

/// @covers: ResourcePolicyBuilder::name
#[test]
fn test_name_sets_policy_name() {
    let policy = ResourcePolicyBuilder::new().name("default").build();
    assert_eq!(policy.name, "default");
}

/// @covers: ResourcePolicyBuilder::timeout_ms
#[test]
fn test_timeout_ms_sets_timeout() {
    let policy = ResourcePolicyBuilder::new().timeout_ms(30_000).build();
    assert_eq!(policy.timeout_ms, 30_000);
}

/// @covers: ResourcePolicyBuilder::output_bytes_cap
#[test]
fn test_output_bytes_cap_sets_cap() {
    let policy = ResourcePolicyBuilder::new()
        .output_bytes_cap(1_048_576)
        .build();
    assert_eq!(policy.output_bytes_cap, 1_048_576);
}

/// @covers: ResourcePolicyBuilder::cpu_time_ms
#[test]
fn test_cpu_time_ms_sets_cpu_limit() {
    let policy = ResourcePolicyBuilder::new().cpu_time_ms(60_000).build();
    assert_eq!(policy.cpu_time_ms, 60_000);
}

/// @covers: ResourcePolicyBuilder::memory_bytes
#[test]
fn test_memory_bytes_sets_memory_limit() {
    let policy = ResourcePolicyBuilder::new()
        .memory_bytes(1_073_741_824)
        .build();
    assert_eq!(policy.memory_bytes, 1_073_741_824);
}

/// @covers: ResourcePolicyBuilder::build
#[test]
fn test_build_returns_policy_with_all_fields() {
    let policy = ResourcePolicyBuilder::new()
        .name("test")
        .timeout_ms(5_000)
        .output_bytes_cap(65_536)
        .cpu_time_ms(1_000)
        .memory_bytes(524_288)
        .build();
    assert_eq!(policy.name, "test");
    assert_eq!(policy.timeout_ms, 5_000);
    assert_eq!(policy.output_bytes_cap, 65_536);
    assert_eq!(policy.cpu_time_ms, 1_000);
    assert_eq!(policy.memory_bytes, 524_288);
}
