//! [`ApplicationConfig`] — top-level application configuration.
//!
//! Maps to `config/application.toml`.

use serde::{Deserialize, Serialize};

use crate::api::types::ObservabilityConfig;

/// Top-level application configuration, loaded from `config/application.toml`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct ApplicationConfig {
    /// Observability configuration.
    pub observability: ObservabilityConfig,
}
