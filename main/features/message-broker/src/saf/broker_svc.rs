//! SAF — message broker and task queue public factory surface.
//!
//! Factory methods are grouped on [`MessageBrokerFactory`] and [`TaskQueueFactory`].
//! Implementation types are returned directly — consumers receive concrete types
//! from the factory methods below and may use them as `impl Trait` at call sites.

#[cfg(feature = "nats")]
use crate::api::broker::broker_error::BrokerError;
use crate::api::broker::message_broker::MessageBroker;
#[cfg(feature = "tokio-rt")]
use crate::api::broker::types::in_memory_message_broker::InMemoryMessageBroker;
#[cfg(feature = "nats")]
use crate::api::task::queue::queue_error::QueueError;
#[cfg(feature = "tokio-rt")]
use crate::api::task::queue::types::in_memory_task_queue::InMemoryTaskQueue;
#[cfg(feature = "nats")]
use crate::api::task::queue::TaskQueue;
use crate::api::types::message_broker_factory::MessageBrokerFactory;
use crate::api::types::task_queue_factory::TaskQueueFactory;
#[cfg(feature = "nats")]
use crate::spi::NatsMessageBroker;
#[cfg(feature = "nats")]
use crate::spi::NatsTaskQueue;
use serde::{Deserialize, Serialize};
#[cfg(feature = "tokio-rt")]
use std::collections::HashMap;
#[cfg(feature = "tokio-rt")]
use std::sync::Arc;
#[cfg(feature = "tokio-rt")]
use tokio::sync::{broadcast, RwLock};

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
        let mut b = swe_edge_configbuilder::ConfigBuilderImpl::new();
        b = b.with_name(env!("CARGO_PKG_NAME"));
        b = b.with_version(env!("CARGO_PKG_VERSION"));
        b
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
            "inmemory" => {
                let broker = InMemoryMessageBroker {
                    channels: Arc::new(RwLock::new(HashMap::<String, broadcast::Sender<_>>::new())),
                };
                Ok(Box::new(broker))
            }
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
    pub fn in_memory() -> InMemoryMessageBroker {
        InMemoryMessageBroker {
            channels: Arc::new(RwLock::new(HashMap::new())),
        }
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
    pub fn in_memory() -> InMemoryTaskQueue {
        let (tx, rx) = tokio::sync::mpsc::channel(1024);
        InMemoryTaskQueue {
            tx: Arc::new(tx),
            rx: Arc::new(tokio::sync::Mutex::new(rx)),
        }
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
