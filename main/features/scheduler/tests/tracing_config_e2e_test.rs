//! Integration tests for [`TracingConfig`].

use swe_edge_runtime_scheduler::TracingConfig;

/// @covers: TracingConfig
#[test]
fn test_tracing_config_default_disabled() {
    let cfg = TracingConfig::default();
    assert!(!cfg.enabled);
}

/// @covers: TracingConfig
#[test]
fn test_tracing_config_fields_are_strings() {
    let cfg = TracingConfig {
        enabled: true,
        format: "json".into(),
        level: "info".into(),
    };
    assert_eq!(cfg.format, "json");
    assert_eq!(cfg.level, "info");
}
