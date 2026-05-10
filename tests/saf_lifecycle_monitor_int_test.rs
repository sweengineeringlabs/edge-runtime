//! Public-API integration tests for saf lifecycle_monitor functions.

use std::sync::Arc;
use edge_proxy::new_null_lifecycle_monitor;
use swe_edge_runtime::{observe_lifecycle_monitor, MetricsProvider};
use swe_observ_metrics::create_local_metrics_backend;

/// @covers: observe_lifecycle_monitor
#[tokio::test]
async fn test_observe_lifecycle_monitor() {
    let provider: Arc<dyn MetricsProvider> = Arc::new(create_local_metrics_backend());
    let inner    = new_null_lifecycle_monitor();
    let observed = observe_lifecycle_monitor(inner, Arc::clone(&provider));
    observed.health().await;
    let snaps = provider.export();
    assert!(
        snaps.iter().any(|s| s.name == "edge_component_health"),
        "expected edge_component_health gauge after health poll, got {snaps:?}"
    );
}
