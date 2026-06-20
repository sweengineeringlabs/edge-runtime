//! [`ApplicationConfig`] — typed configuration for the message broker application.

use serde::{Deserialize, Serialize};

use crate::api::config::types::broker_backend_config::BrokerBackendConfig;

/// Typed configuration for the `swe-edge-runtime-message-broker` application.
///
/// Loaded from `config/application.toml` via the `[message_broker]` section.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct ApplicationConfig {
    /// Configuration for the message broker backend.
    pub message_broker: BrokerBackendConfig,
}
