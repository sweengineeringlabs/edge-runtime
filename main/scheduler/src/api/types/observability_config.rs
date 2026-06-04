//! [`ObservabilityConfig`] — observability-related configuration.

use serde::{Deserialize, Serialize};

use crate::api::types::TracingConfig;

/// Observability-related configuration.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct ObservabilityConfig {
    /// Tracing configuration.
    pub tracing: TracingConfig,
}
