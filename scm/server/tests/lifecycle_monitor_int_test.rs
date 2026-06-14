//! Integration tests for the api/monitor lifecycle_monitor module.

use edge_proxy::ProxySvc;
use std::sync::Arc;
use swe_edge_runtime::{MetricsProvider, ServerMonitor};
use swe_observ_metrics::create_local_metrics_backend;

/// @covers: observe
#[tokio::test]
async fn test_observe_wraps_monitor_and_emits_health_gauge_happy() {
    let provider: Arc<dyn MetricsProvider> = Arc::new(create_local_metrics_backend());
    let inner = ProxySvc::new_null_lifecycle_monitor();
    let observed = ServerMonitor::observe(inner, Arc::clone(&provider));
    observed.health().await;
    let snaps = provider.export();
    assert!(
        snaps.iter().any(|s| s.name == "edge_component_health"),
        "expected edge_component_health gauge after health poll, got {snaps:?}"
    );
}

#[tokio::test]
async fn test_null_monitor_health_returns_empty_components_error() {
    let monitor = ProxySvc::new_null_lifecycle_monitor();
    let report = monitor.health().await;
    assert!(
        report.components.is_empty(),
        "null monitor has no components"
    );
}

#[tokio::test]
async fn test_null_monitor_shutdown_is_safe_edge() {
    let monitor = ProxySvc::new_null_lifecycle_monitor();
    let result = monitor.shutdown().await;
    assert!(result.is_ok(), "null monitor shutdown must not fail");
}
