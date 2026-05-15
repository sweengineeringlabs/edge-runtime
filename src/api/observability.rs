//! Tracing subscriber initialisation — opt-in via the `observability` feature.

/// Install a `tracing-subscriber` driven by a [`TracingConfig`].
///
/// Respects `RUST_LOG` for filter level (takes precedence over `config.level`
/// and `config.filter`). Safe to call more than once — subsequent calls are
/// silent no-ops because the global subscriber is already set.
///
/// Does nothing when `config.enabled` is `false`.
#[cfg(feature = "observability")]
pub fn init_tracing(config: &crate::api::config::TracingConfig) {
    use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};
    use crate::api::tracing_format::TracingFormat;

    if !config.enabled { return; }

    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| {
            let base = config.level.as_str();
            let directive = match &config.filter {
                Some(f) if !f.is_empty() => format!("{base},{f}"),
                _ => base.to_owned(),
            };
            EnvFilter::new(directive)
        });

    match config.format {
        TracingFormat::Json => {
            let _ = tracing_subscriber::registry()
                .with(filter)
                .with(
                    fmt::layer()
                        .json()
                        .flatten_event(true)
                        .with_current_span(true)
                        .with_span_list(false),
                )
                .try_init();
        }
        TracingFormat::Pretty => {
            let _ = tracing_subscriber::registry()
                .with(filter)
                .with(fmt::layer().pretty())
                .try_init();
        }
    }
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "observability")]
    use super::init_tracing;
    #[cfg(feature = "observability")]
    use crate::api::config::{TracingConfig, TracingLevel};
    #[cfg(feature = "observability")]
    use crate::api::tracing_format::TracingFormat;

    /// @covers: init_tracing
    #[cfg(feature = "observability")]
    #[test]
    fn test_init_tracing_json_does_not_panic() {
        let cfg = TracingConfig { format: TracingFormat::Json, ..TracingConfig::default() };
        init_tracing(&cfg);
    }

    /// @covers: init_tracing
    #[cfg(feature = "observability")]
    #[test]
    fn test_init_tracing_pretty_does_not_panic() {
        let cfg = TracingConfig { format: TracingFormat::Pretty, ..TracingConfig::default() };
        init_tracing(&cfg);
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
        let cfg = TracingConfig { enabled: false, ..TracingConfig::default() };
        init_tracing(&cfg); // must not install a subscriber
    }

    /// @covers: init_tracing
    #[cfg(feature = "observability")]
    #[test]
    fn test_init_tracing_with_custom_level_does_not_panic() {
        let cfg = TracingConfig { level: TracingLevel::Warn, ..TracingConfig::default() };
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
