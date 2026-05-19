/// Prometheus-style metric name emitted by `MetricsLifecycleMonitor` for
/// each component health poll.
pub const LIFECYCLE_HEALTH_GAUGE: &str = "edge_component_health";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lifecycle_health_gauge_name_is_non_empty() {
        assert!(!LIFECYCLE_HEALTH_GAUGE.is_empty());
    }
}
