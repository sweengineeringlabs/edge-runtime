//! API declarations for the Prometheus metrics HTTP handler.

use crate::api::monitor::SharedCounters;

/// Contract for serving the Prometheus text exposition endpoint.
pub trait MetricsExporter: Send + Sync {
    /// Bind address and path are determined by [`crate::api::monitor::MetricsConfig`].
    fn counters(&self) -> &SharedCounters;
    fn path(&self) -> &str;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use swe_observ_metrics::create_local_metrics_backend;
    use crate::api::monitor::TrafficCounters;

    struct StubExporter {
        counters: SharedCounters,
        path:     String,
    }
    impl MetricsExporter for StubExporter {
        fn counters(&self) -> &SharedCounters { &self.counters }
        fn path(&self)     -> &str            { &self.path }
    }

    #[test]
    fn test_metrics_exporter_is_object_safe() {
        fn _assert(_: &dyn MetricsExporter) {}
        let e = StubExporter {
            counters: Arc::new(TrafficCounters::new(Arc::new(create_local_metrics_backend()))),
            path: "/metrics".into(),
        };
        _assert(&e);
        assert_eq!(e.path(), "/metrics");
    }
}
