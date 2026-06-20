//! API declarations for the Prometheus metrics HTTP handler.

use crate::api::metrics::types::metrics_config::MetricsConfig;

/// Contract for serving the Prometheus text exposition endpoint.
pub trait MetricsExporter: Send + Sync {
    /// Bind address and path are determined by [`MetricsConfig`](crate::MetricsConfig).
    fn config(&self) -> &MetricsConfig;
    /// Export current metric snapshots.
    fn export(&self) -> Vec<swe_observ_metrics::MetricSnapshot>;
}
