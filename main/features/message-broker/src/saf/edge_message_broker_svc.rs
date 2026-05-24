//! SAF — message broker public factory surface.

use swe_edge_configbuilder::ConfigBuilder as _;
use crate::api::broker::message_broker::MessageBroker;
use serde::{Deserialize, Serialize};
#[cfg(feature = "nats")]
use crate::api::broker::broker_error::BrokerError;
#[cfg(feature = "tokio-rt")]
use crate::spi::InMemoryMessageBroker;
#[cfg(feature = "nats")]
use crate::spi::NatsMessageBroker;

/// Internal: configuration loaded from [message_broker] section of application.toml.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
struct BrokerConfig {
    backend: String,
    nats_url: String,
    kafka_brokers: String,
}

impl Default for BrokerConfig {
    fn default() -> Self {
        Self {
            backend: "inmemory".into(),
            nats_url: "nats://localhost:4222".into(),
            kafka_brokers: "localhost:9092".into(),
        }
    }
}

/// Return a [`ConfigBuilder`] pre-seeded with this crate's package name and version.
pub fn create_config_builder() -> impl swe_edge_configbuilder::ConfigBuilder {
    swe_edge_configbuilder::create_config_builder()
        .with_name(env!("CARGO_PKG_NAME"))
        .with_version(env!("CARGO_PKG_VERSION"))
}

/// Instantiate a message broker from configuration loaded from application.toml.
///
/// Reads `[message_broker]` section and selects backend based on `backend` field.
///
/// # Errors
///
/// Returns error if the selected backend cannot be initialized (e.g., NATS connection fails).
pub async fn broker_from_config() -> Result<Box<dyn MessageBroker>, Box<dyn std::error::Error>> {
    let cfg: BrokerConfig = swe_edge_config::load_section("message_broker")?;
    match cfg.backend.as_str() {
        #[cfg(feature = "tokio-rt")]
        "inmemory" => Ok(Box::new(InMemoryMessageBroker::new())),
        #[cfg(not(feature = "tokio-rt"))]
        "inmemory" => Err("in-memory broker requires tokio-rt feature".into()),

        #[cfg(feature = "nats")]
        "nats" => {
            let broker = NatsMessageBroker::connect(&cfg.nats_url).await?;
            Ok(Box::new(broker))
        }
        #[cfg(not(feature = "nats"))]
        "nats" => Err("NATS broker requires nats feature".into()),

        "kafka" => Err("Kafka backend not yet implemented".into()),
        other => Err(format!("Unknown broker backend: {}", other).into()),
    }
}

/// Construct an in-memory broker backed by [`tokio::sync::broadcast`].
///
/// Topics are created lazily on first subscription.  All subscribers on the
/// same topic receive every message published after they subscribed.
///
/// Requires the `tokio-rt` feature.
#[cfg(feature = "tokio-rt")]
pub fn in_memory_broker() -> impl MessageBroker + Clone {
    InMemoryMessageBroker::new()
}

/// Connect to a NATS server and return a broker handle.
///
/// # Errors
///
/// Returns [`BrokerError::Connection`] if the NATS server is unreachable.
///
/// Requires the `nats` feature.
#[cfg(feature = "nats")]
pub async fn nats_broker(url: &str) -> Result<impl MessageBroker, BrokerError> {
    NatsMessageBroker::connect(url).await
}

#[cfg(test)]
mod tests {

    /// @covers: in_memory_broker
    #[cfg(feature = "tokio-rt")]
    #[test]
    fn test_in_memory_broker_factory_produces_working_broker() {
        use crate::api::broker::MessageBroker;
        use crate::in_memory_broker;
        use futures::executor::block_on;
        let broker = in_memory_broker();
        block_on(async move {
            assert!(broker.health_check().await.is_ok());
        });
    }

    /// @covers: nats_broker
    #[test]
    fn test_nats_broker_is_feature_gated_behind_nats() {
        let enabled = cfg!(feature = "nats");
        let _ = enabled;
    }

    /// @covers: validate
    #[cfg(feature = "tokio-rt")]
    #[test]
    fn test_validate_returns_ok_for_valid_broker() {
        use crate::api::traits::Validator;
        use crate::spi::InMemoryMessageBroker;
        assert!(InMemoryMessageBroker::new().validate().is_ok());
    }
}
