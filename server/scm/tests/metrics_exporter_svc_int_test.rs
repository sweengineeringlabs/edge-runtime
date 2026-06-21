//! Integration tests for the metrics_exporter_svc SAF surface.

use std::sync::Arc;
use swe_edge_runtime::{MetricsConfig, MetricsExporter, METRICS_EXPORTER_SVC};
use swe_observ_metrics::{MetricSnapshot, MetricsProvider};

struct TestExporter {
    config: MetricsConfig,
    snapshots: Vec<MetricSnapshot>,
}

impl MetricsExporter for TestExporter {
    fn config(&self) -> &MetricsConfig {
        &self.config
    }
    fn export(&self) -> Vec<MetricSnapshot> {
        self.snapshots.clone()
    }
}

fn make_exporter(bind: &str, path: &str, snapshots: Vec<MetricSnapshot>) -> TestExporter {
    TestExporter {
        config: MetricsConfig {
            bind: bind.into(),
            path: path.into(),
        },
        snapshots,
    }
}

fn snapshot_from_provider(provider: &dyn MetricsProvider) -> Vec<MetricSnapshot> {
    provider.export()
}

/// @covers: METRICS_EXPORTER_SVC
#[test]
fn test_metrics_exporter_svc_slug_is_correct_happy() {
    assert_eq!(METRICS_EXPORTER_SVC, "metrics_exporter");
}

// ── MetricsExporter::config ───────────────────────────────────────────────────

#[test]
fn test_config_returns_correct_bind_address_happy() {
    let exporter = make_exporter("0.0.0.0:9091", "/metrics", vec![]);
    assert_eq!(exporter.config().bind, "0.0.0.0:9091");
}

#[test]
fn test_config_path_does_not_equal_bind_error() {
    let exporter = make_exporter("0.0.0.0:9090", "/metrics", vec![]);
    assert_ne!(exporter.config().bind, exporter.config().path);
}

#[test]
fn test_config_path_starts_with_slash_edge() {
    let exporter = make_exporter("0.0.0.0:9090", "/metrics", vec![]);
    assert!(exporter.config().path.starts_with('/'));
}

// ── MetricsExporter::export ───────────────────────────────────────────────────

#[test]
fn test_export_returns_empty_when_no_snapshots_happy() {
    let exporter = make_exporter("0.0.0.0:9090", "/metrics", vec![]);
    assert!(exporter.export().is_empty());
}

#[test]
fn test_export_returns_snapshots_when_populated_error() {
    use swe_observ_metrics::create_local_metrics_backend;
    let provider = Arc::new(create_local_metrics_backend());
    provider.record_counter("req_total", 7.0, &[]);
    let snaps = snapshot_from_provider(&*provider);
    let exporter = make_exporter("0.0.0.0:9090", "/metrics", snaps);
    assert!(!exporter.export().is_empty());
}

#[test]
fn test_export_snapshot_names_match_recorded_metrics_edge() {
    use swe_observ_metrics::create_local_metrics_backend;
    let provider = Arc::new(create_local_metrics_backend());
    provider.record_counter("edge_hits", 1.0, &[]);
    let snaps = snapshot_from_provider(&*provider);
    let exporter = make_exporter("0.0.0.0:9090", "/metrics", snaps);
    let exported = exporter.export();
    let names: Vec<&str> = exported.iter().map(|s| s.name.as_str()).collect();
    assert!(
        names.contains(&"edge_hits"),
        "expected edge_hits in: {names:?}"
    );
}
