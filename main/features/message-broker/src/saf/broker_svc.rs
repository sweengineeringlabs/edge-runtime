//! SAF — message broker and task queue public factory surface.
//!
//! Factory methods are grouped on [`MessageBrokerFactory`] and [`TaskQueueFactory`].
//! Implementation types are never exposed directly — consumers receive opaque
//! `impl Trait` values from the factory methods below.

#[cfg(feature = "nats")]
use crate::api::broker::broker_error::BrokerError;
use crate::api::broker::message_broker::MessageBroker;
#[cfg(feature = "nats")]
use crate::api::task::queue::queue_error::QueueError;
#[cfg(any(feature = "tokio-rt", feature = "nats"))]
use crate::api::task::queue::TaskQueue;
use crate::api::types::message_broker_factory::MessageBrokerFactory;
use crate::api::types::task_queue_factory::TaskQueueFactory;
#[cfg(feature = "tokio-rt")]
use crate::spi::InMemoryMessageBroker;
#[cfg(feature = "tokio-rt")]
use crate::spi::InMemoryTaskQueue;
#[cfg(feature = "nats")]
use crate::spi::NatsMessageBroker;
#[cfg(feature = "nats")]
use crate::spi::NatsTaskQueue;
use serde::{Deserialize, Serialize};
use swe_edge_configbuilder::ConfigLoaderFactory;

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

impl MessageBrokerFactory {
    /// Return a [`swe_edge_configbuilder::ConfigBuilderImpl`] pre-seeded with this crate's package name and version.
    pub fn create_config_builder() -> swe_edge_configbuilder::ConfigBuilderImpl {
        ConfigLoaderFactory::create_config_builder()
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
    pub async fn broker_from_config() -> Result<Box<dyn MessageBroker>, Box<dyn std::error::Error>>
    {
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
            other => Err(format!("Unknown broker backend: {other}").into()),
        }
    }

    /// Construct an in-memory broker backed by [`tokio::sync::broadcast`].
    ///
    /// Requires the `tokio-rt` feature.
    #[cfg(feature = "tokio-rt")]
    pub fn in_memory() -> impl MessageBroker + Clone {
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
    pub async fn nats(url: &str) -> Result<impl MessageBroker, BrokerError> {
        NatsMessageBroker::connect(url).await
    }
}

impl TaskQueueFactory {
    /// Construct an in-memory task queue backed by [`tokio::sync::mpsc`].
    ///
    /// Requires the `tokio-rt` feature.
    #[cfg(feature = "tokio-rt")]
    pub fn in_memory() -> impl TaskQueue + Clone {
        InMemoryTaskQueue::new()
    }

    /// Connect to a NATS server and return a task queue handle.
    ///
    /// # Errors
    ///
    /// Returns [`QueueError::Connection`] if the NATS server is unreachable.
    ///
    /// Requires the `nats` feature.
    #[cfg(feature = "nats")]
    pub async fn nats(
        nats_url: &str,
        stream_name: String,
        consumer_group: String,
    ) -> Result<impl TaskQueue, QueueError> {
        let connection = async_nats::connect(nats_url)
            .await
            .map_err(|e| QueueError::Connection(e.to_string()))?;

        let jetstream_context = async_nats::jetstream::new(connection);

        NatsTaskQueue::new(jetstream_context, stream_name, consumer_group).await
    }
}
