use std::sync::Arc;

use async_trait::async_trait;
use edge_proxy::{HealthReport, HealthStatus, LifecycleError, LifecycleMonitor};
use swe_observ_metrics::MetricsProvider;

/// Wraps any [`LifecycleMonitor`]; emits an `edge_component_health` gauge for
/// every component in the health report after each poll.
///
/// Values: `1.0` = Healthy, `0.5` = Degraded, `0.0` = Unhealthy.
pub(crate) struct MetricsLifecycleMonitor {
    inner: Arc<dyn LifecycleMonitor>,
    provider: Arc<dyn MetricsProvider>,
}

impl crate::api::monitor::LifecycleObserver for MetricsLifecycleMonitor {}

impl MetricsLifecycleMonitor {
    pub(crate) fn new(
        inner: Arc<dyn LifecycleMonitor>,
        provider: Arc<dyn MetricsProvider>,
    ) -> Self {
        Self { inner, provider }
    }

    fn score(status: HealthStatus) -> f64 {
        match status {
            HealthStatus::Healthy => 1.0,
            HealthStatus::Degraded => 0.5,
            HealthStatus::Unhealthy => 0.0,
        }
    }
}

#[async_trait]
impl LifecycleMonitor for MetricsLifecycleMonitor {
    async fn health(&self) -> HealthReport {
        let report = self.inner.health().await;
        for component in &report.components {
            self.provider.record_gauge(
                "edge_component_health",
                Self::score(component.status),
                &[("component", component.id.as_str())],
            );
        }
        self.provider.record_gauge(
            "edge_component_health",
            Self::score(report.overall),
            &[("component", "overall")],
        );
        report
    }

    async fn start_background_tasks(&self) {
        self.inner.start_background_tasks().await
    }

    async fn shutdown(&self) -> Result<(), LifecycleError> {
        self.inner.shutdown().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use edge_proxy::{new_null_lifecycle_monitor, HealthStatus};
    use swe_observ_metrics::create_local_metrics_backend;

    fn provider() -> Arc<dyn MetricsProvider> {
        Arc::new(create_local_metrics_backend())
    }

    #[test]
    fn test_new_creates_monitor_without_panic() {
        let _m = MetricsLifecycleMonitor::new(new_null_lifecycle_monitor(), provider());
    }

    #[tokio::test]
    async fn test_health_records_healthy_component_as_one() {
        let p = provider();
        let m = MetricsLifecycleMonitor::new(new_null_lifecycle_monitor(), Arc::clone(&p));
        m.health().await;
        let snaps = p.export();
        assert!(
            snaps
                .iter()
                .any(|s| s.name == "edge_component_health" && s.value == 1.0),
            "expected edge_component_health=1.0, got {snaps:?}"
        );
    }

    #[tokio::test]
    async fn test_health_records_unhealthy_overall_after_shutdown() {
        let inner = new_null_lifecycle_monitor();
        inner.shutdown().await.ok();
        let p = provider();
        let m = MetricsLifecycleMonitor::new(inner, Arc::clone(&p));
        m.health().await;
        let snaps = p.export();
        assert!(
            snaps.iter().any(|s| s.name == "edge_component_health"),
            "expected edge_component_health gauge, got {snaps:?}"
        );
        assert!(
            snaps.iter().any(|s| s.name == "edge_component_health"
                && s.value == HealthStatus::Unhealthy as i32 as f64
                || s.value == 0.0),
            "expected unhealthy score after shutdown, got {snaps:?}"
        );
    }

    #[tokio::test]
    async fn test_shutdown_delegates_to_inner() {
        let m = MetricsLifecycleMonitor::new(new_null_lifecycle_monitor(), provider());
        assert!(m.shutdown().await.is_ok());
    }
}
