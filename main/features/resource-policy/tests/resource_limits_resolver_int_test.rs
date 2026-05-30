//! Integration tests for ResourceLimitsResolver.

use swe_edge_runtime_resource_policy::{ResourceLimits, ResourceLimitsResolver, ResourcePolicy};

fn floor() -> ResourcePolicy {
    ResourcePolicy {
        name: "floor".into(),
        timeout_ms: 30_000,
        output_bytes_cap: 1_048_576,
        cpu_time_ms: 0,
        memory_bytes: 0,
    }
}

/// @covers: ResourceLimitsResolver::new, ResourceLimitsResolver::resolve_with_defaults
#[test]
fn test_resolver_no_layers_returns_defaults() {
    let resolved = ResourceLimitsResolver::new().resolve_with_defaults(&floor());
    assert_eq!(resolved.timeout_ms, 30_000);
}

/// @covers: ResourceLimitsResolver::layer, ResourceLimitsResolver::resolve_with_defaults
#[test]
fn test_resolver_layer_overrides_floor() {
    let layer = ResourceLimits {
        timeout_ms: Some(5_000),
        ..Default::default()
    };
    let resolved = ResourceLimitsResolver::new()
        .layer(layer)
        .resolve_with_defaults(&floor());
    assert_eq!(resolved.timeout_ms, 5_000);
}

/// @covers: ResourceLimitsResolver::resolve_with_defaults
#[test]
fn test_resolver_earlier_layer_has_higher_priority() {
    let high = ResourceLimits {
        timeout_ms: Some(1_000),
        ..Default::default()
    };
    let low = ResourceLimits {
        timeout_ms: Some(9_000),
        ..Default::default()
    };
    let resolved = ResourceLimitsResolver::new()
        .layer(high)
        .layer(low)
        .resolve_with_defaults(&floor());
    assert_eq!(resolved.timeout_ms, 1_000);
}
