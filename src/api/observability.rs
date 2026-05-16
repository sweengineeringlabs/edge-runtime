//! Tracing subscriber initialisation — delegates to swe-edge-observ-config.

#[cfg(feature = "observability")]
use swe_edge_observ_config::TracingConfig;

/// Install a `tracing-subscriber` driven by `config`.
///
/// Requires the `observability` feature. Idempotent — safe to call multiple
/// times. Does nothing when `config.enabled` is `false`. `RUST_LOG` overrides
/// `config.level` and `config.filter`.
#[cfg(feature = "observability")]
pub fn init_tracing(config: &TracingConfig) {
    swe_edge_observ_config::init_tracing(config);
}

#[cfg(test)]
mod tests {
    use super::*;
    use swe_edge_observ_config::{TracingFormat, TracingLevel};

    /// @covers: init_tracing
    #[cfg(feature = "observability")]
    #[test]
    fn test_init_tracing_json_does_not_panic() {
        let cfg = TracingConfig {
            format: TracingFormat::Json,
            ..TracingConfig::default()
        };
        init_tracing(&cfg);
    }

    /// @covers: init_tracing
    #[cfg(feature = "observability")]
    #[test]
    fn test_init_tracing_pretty_does_not_panic() {
        init_tracing(&TracingConfig::default());
    }

    /// @covers: init_tracing
    #[cfg(feature = "observability")]
    #[test]
    fn test_init_tracing_called_twice_does_not_panic() {
        init_tracing(&TracingConfig::default());
        init_tracing(&TracingConfig::default());
    }

    /// @covers: init_tracing
    #[cfg(feature = "observability")]
    #[test]
    fn test_init_tracing_disabled_is_noop() {
        let cfg = TracingConfig {
            enabled: false,
            ..TracingConfig::default()
        };
        init_tracing(&cfg);
    }

    /// @covers: init_tracing
    #[cfg(feature = "observability")]
    #[test]
    fn test_init_tracing_with_custom_level_does_not_panic() {
        let cfg = TracingConfig {
            level: TracingLevel::Warn,
            ..TracingConfig::default()
        };
        init_tracing(&cfg);
    }

    /// @covers: init_tracing
    #[cfg(feature = "observability")]
    #[test]
    fn test_init_tracing_with_filter_does_not_panic() {
        let cfg = TracingConfig {
            filter: Some("tower=warn".into()),
            ..TracingConfig::default()
        };
        init_tracing(&cfg);
    }
}
