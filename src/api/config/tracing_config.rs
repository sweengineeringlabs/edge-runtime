//! `TracingConfig` — TOML-driven tracing subscriber configuration.

use serde::{Deserialize, Serialize};

use crate::api::config::tracing_level::TracingLevel;
use crate::api::tracing_format::TracingFormat;

/// Configuration for the tracing subscriber.
///
/// Mapped from the `[observability.tracing]` TOML section.
/// `RUST_LOG` always takes precedence over `level` and `filter`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct TracingConfig {
    /// Install the tracing subscriber on `serve()`.  Default: `true`.
    pub enabled: bool,
    /// Output format: `"pretty"` (dev) or `"json"` (prod).  Default: `"pretty"`.
    pub format: TracingFormat,
    /// Minimum log level.  Ignored when `RUST_LOG` is set.  Default: `"info"`.
    pub level: TracingLevel,
    /// Optional module-level filter string (e.g. `"my_crate=debug,tower=warn"`).
    /// Appended after `level` when both are present.  Ignored when `RUST_LOG` is set.
    pub filter: Option<String>,
}

impl Default for TracingConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            format:  TracingFormat::Pretty,
            level:   TracingLevel::Info,
            filter:  None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tracing_config_default_values() {
        let cfg = TracingConfig::default();
        assert!(cfg.enabled);
        assert_eq!(cfg.format, TracingFormat::Pretty);
        assert_eq!(cfg.level, TracingLevel::Info);
        assert!(cfg.filter.is_none());
    }

    #[test]
    fn test_tracing_config_deserializes_all_fields_from_toml() {
        let toml = r#"
            enabled = true
            format  = "json"
            level   = "debug"
            filter  = "my_crate=trace"
        "#;
        let cfg: TracingConfig = toml::from_str(toml).unwrap();
        assert!(cfg.enabled);
        assert_eq!(cfg.format, TracingFormat::Json);
        assert_eq!(cfg.level, TracingLevel::Debug);
        assert_eq!(cfg.filter.as_deref(), Some("my_crate=trace"));
    }

    #[test]
    fn test_tracing_config_deserializes_partial_toml_using_defaults() {
        let cfg: TracingConfig = toml::from_str(r#"level = "warn""#).unwrap();
        assert!(cfg.enabled);
        assert_eq!(cfg.level, TracingLevel::Warn);
        assert_eq!(cfg.format, TracingFormat::Pretty);
    }

    #[test]
    fn test_tracing_config_disabled_field_deserializes() {
        let cfg: TracingConfig = toml::from_str("enabled = false").unwrap();
        assert!(!cfg.enabled);
    }
}
