//! [`ApplicationConfig`] — typed configuration for the message broker application.

use serde::{Deserialize, Serialize};

use crate::api::types::broker_backend_config::BrokerBackendConfig;

/// Typed configuration for the `swe-edge-runtime-message-broker` application.
///
/// Loaded from `config/application.toml` via the `[message_broker]` section.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
#[expect(
    dead_code,
    reason = "SEA api/ anchor — exported for consumers, not used internally"
)]
pub struct ApplicationConfig {
    /// Configuration for the message broker backend.
    pub message_broker: BrokerBackendConfig,
}
