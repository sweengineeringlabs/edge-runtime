//! `TracingLevel` — log level selector for the tracing subscriber.

use serde::{Deserialize, Serialize};

/// Minimum log level for the tracing subscriber.
///
/// Overridden at runtime by the `RUST_LOG` environment variable.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum TracingLevel {
    /// `tracing::Level::TRACE` — every span and event.
    Trace,
    /// `tracing::Level::DEBUG` — debug-level diagnostics.
    Debug,
    /// `tracing::Level::INFO` — normal operational messages.
    #[default]
    Info,
    /// `tracing::Level::WARN` — recoverable anomalies.
    Warn,
    /// `tracing::Level::ERROR` — errors that require attention.
    Error,
}

impl TracingLevel {
    pub(crate) fn as_str(&self) -> &'static str {
        match self {
            Self::Trace => "trace",
            Self::Debug => "debug",
            Self::Info  => "info",
            Self::Warn  => "warn",
            Self::Error => "error",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tracing_level_default_is_info() {
        assert_eq!(TracingLevel::default(), TracingLevel::Info);
    }

    #[test]
    fn test_tracing_level_as_str_matches_lowercase_variant_name() {
        assert_eq!(TracingLevel::Trace.as_str(), "trace");
        assert_eq!(TracingLevel::Debug.as_str(), "debug");
        assert_eq!(TracingLevel::Info.as_str(),  "info");
        assert_eq!(TracingLevel::Warn.as_str(),  "warn");
        assert_eq!(TracingLevel::Error.as_str(), "error");
    }

    #[test]
    fn test_tracing_level_deserializes_from_lowercase_toml_string() {
        #[derive(serde::Deserialize)]
        struct Wrapper { level: TracingLevel }
        let w: Wrapper = toml::from_str(r#"level = "warn""#).unwrap();
        assert_eq!(w.level, TracingLevel::Warn);
    }

    #[test]
    fn test_tracing_level_serializes_to_lowercase_string() {
        #[derive(serde::Serialize)]
        struct W { level: TracingLevel }
        let s = toml::to_string(&W { level: TracingLevel::Error }).unwrap();
        assert!(s.contains("\"error\""), "serialized: {s}");
    }
}
