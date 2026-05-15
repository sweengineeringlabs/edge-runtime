//! `ObservabilityConfig` — top-level observability configuration.

use serde::{Deserialize, Serialize};

use crate::api::config::tracing_config::TracingConfig;

/// Top-level observability configuration.
///
/// Mapped from the `[observability]` TOML section.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ObservabilityConfig {
    /// Tracing subscriber configuration (`[observability.tracing]`).
    pub tracing: TracingConfig,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::config::tracing_level::TracingLevel;

    #[test]
    fn test_observability_config_default_has_tracing_enabled() {
        assert!(ObservabilityConfig::default().tracing.enabled);
    }

    #[test]
    fn test_observability_config_deserializes_nested_tracing_section() {
        let toml = r#"
            [tracing]
            level   = "debug"
            format  = "json"
        "#;
        let cfg: ObservabilityConfig = toml::from_str(toml).unwrap();
        assert_eq!(cfg.tracing.level, TracingLevel::Debug);
    }

    #[test]
    fn test_observability_config_empty_toml_uses_all_defaults() {
        let cfg: ObservabilityConfig = toml::from_str("").unwrap();
        assert!(cfg.tracing.enabled);
        assert_eq!(cfg.tracing.level, TracingLevel::Info);
    }
}
