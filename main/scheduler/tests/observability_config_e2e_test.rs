//! Integration tests for [`ObservabilityConfig`].

use swe_edge_runtime_scheduler::ObservabilityConfig;

/// @covers: ObservabilityConfig
#[test]
fn test_observability_config_default_is_valid() {
    let cfg = ObservabilityConfig::default();
    assert!(!cfg.tracing.enabled);
}

/// @covers: ObservabilityConfig
#[test]
fn test_observability_config_tracing_is_accessible() {
    let cfg = ObservabilityConfig::default();
    assert!(cfg.tracing.level.is_empty());
}
