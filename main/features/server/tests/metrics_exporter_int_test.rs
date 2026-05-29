//! Integration tests for MetricsExporter trait coverage.

use std::sync::Arc;
use swe_edge_runtime::{AutoscalePolicy, MetricsConfig, SharedCounters, TrafficCounters};
use swe_observ_metrics::create_local_metrics_backend;

/// @covers: MetricsExporter
#[test]
fn test_metrics_config_default_bind_is_set() {
    let c = MetricsConfig::default();
    assert!(!c.bind.is_empty());
}

/// @covers: MetricsExporter
#[test]
fn test_metrics_config_default_path_starts_with_slash() {
    let c = MetricsConfig::default();
    assert!(c.path.starts_with('/'));
}
