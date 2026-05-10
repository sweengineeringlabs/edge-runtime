use std::sync::Arc;

use edge_proxy::LifecycleMonitor;
use swe_observ_metrics::MetricsProvider;

use crate::core::monitor::MetricsLifecycleMonitor;

/// Wrap any [`LifecycleMonitor`] with health-state gauge recording.
///
/// After every `health()` call each component's status is emitted as
/// `edge_component_health` (1.0 = Healthy, 0.5 = Degraded, 0.0 = Unhealthy).
pub fn observe_lifecycle_monitor(
    inner:    Arc<dyn LifecycleMonitor>,
    provider: Arc<dyn MetricsProvider>,
) -> Arc<dyn LifecycleMonitor> {
    Arc::new(MetricsLifecycleMonitor::new(inner, provider))
}

#[cfg(test)]
mod tests {
    use super::*;
    use edge_proxy::new_null_lifecycle_monitor;
    use swe_observ_metrics::create_local_metrics_backend;

    /// @covers: observe_lifecycle_monitor
    #[test]
    fn test_observe_lifecycle_monitor_returns_arc_lifecycle_monitor() {
        let inner    = new_null_lifecycle_monitor();
        let provider: Arc<dyn MetricsProvider> = Arc::new(create_local_metrics_backend());
        let _observed = observe_lifecycle_monitor(inner, provider);
    }

    /// @covers: observe_lifecycle_monitor
    #[tokio::test]
    async fn test_observe_lifecycle_monitor_delegates_health_to_inner() {
        let inner    = new_null_lifecycle_monitor();
        let provider: Arc<dyn MetricsProvider> = Arc::new(create_local_metrics_backend());
        let observed = observe_lifecycle_monitor(inner, provider);
        let report   = observed.health().await;
        assert!(matches!(report.overall, edge_proxy::HealthStatus::Healthy));
    }
}
