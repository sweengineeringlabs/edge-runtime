//! [`TracingConfig`] — tracing-specific configuration.

use serde::{Deserialize, Serialize};

/// Tracing configuration.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct TracingConfig {
    /// Whether tracing is enabled.
    pub enabled: bool,
    /// Output format: "pretty" or "json".
    pub format: String,
    /// Log level: "trace", "debug", "info", "warn", or "error".
    pub level: String,
}
