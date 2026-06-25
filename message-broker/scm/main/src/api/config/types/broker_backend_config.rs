//! [`BrokerBackendConfig`] — message broker backend configuration.

use serde::{Deserialize, Serialize};

/// Configuration for the message broker backend selection.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct BrokerBackendConfig {
    /// Backend type: `inmemory`, `nats`, or `kafka`.
    pub backend: String,
    /// NATS server URL (used when `backend = "nats"`).
    pub nats_url: String,
    /// Kafka broker addresses (used when `backend = "kafka"`).
    pub kafka_brokers: String,
}
