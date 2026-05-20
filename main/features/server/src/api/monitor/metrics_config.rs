//! `MetricsConfig` — Prometheus endpoint bind address and path.

use serde::{Deserialize, Serialize};

/// Configuration for the Prometheus metrics endpoint.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct MetricsConfig {
    /// Bind address for the metrics server (separate from `http_bind`).
    pub bind: String,
    /// Path served by the metrics endpoint.
    pub path: String,
}

impl Default for MetricsConfig {
    fn default() -> Self {
        Self {
            bind: "0.0.0.0:9090".into(),
            path: "/metrics".into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metrics_config_default_has_expected_values() {
        let c = MetricsConfig::default();
        assert_eq!(c.bind, "0.0.0.0:9090");
        assert_eq!(c.path, "/metrics");
    }
}
