//! Tracing subscriber initialisation — opt-in via the `observability` feature.

/// Output format for the tracing subscriber.
pub enum TracingFormat {
    /// Structured JSON lines — for prod log aggregators (Loki, Datadog, CloudWatch).
    ///
    /// Each log line is a self-contained JSON object. Current span fields
    /// (`trace_id`, `session_id`, `agent_id` set by `justobserv_context`) are
    /// flattened into every event so they appear inline rather than nested.
    Json,
    /// Human-readable colour output — for local development.
    Pretty,
}

/// Install a `tracing-subscriber` that surfaces `justobserv` context fields
/// (`trace_id`, `session_id`, `agent_id`) in every log line.
///
/// Respects `RUST_LOG` for filter level (defaults to `info`). Safe to call
/// more than once — subsequent calls are silent no-ops because the global
/// subscriber is already set.
#[cfg(feature = "observability")]
pub fn init_tracing(format: TracingFormat) {
    use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));

    match format {
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

#[cfg(all(test, feature = "observability"))]
mod tests {
    use super::*;

    /// @covers: init_tracing — Json
    #[test]
    fn test_init_tracing_json_does_not_panic() {
        init_tracing(TracingFormat::Json);
    }

    /// @covers: init_tracing — Pretty
    #[test]
    fn test_init_tracing_pretty_does_not_panic() {
        init_tracing(TracingFormat::Pretty);
    }

    /// @covers: init_tracing — idempotent
    #[test]
    fn test_init_tracing_called_twice_does_not_panic() {
        init_tracing(TracingFormat::Json);
        init_tracing(TracingFormat::Pretty);
    }
}
