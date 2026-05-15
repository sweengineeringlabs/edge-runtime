//! `TracingFormat` — output format selector for the tracing subscriber.

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tracing_format_variants_are_constructible() {
        let _json   = TracingFormat::Json;
        let _pretty = TracingFormat::Pretty;
    }
}
