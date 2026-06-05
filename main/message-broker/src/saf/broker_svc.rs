//! SAF — message broker and task queue public factory surface.
//!
//! Factory methods are grouped on [`MessageBrokerFactory`] and [`TaskQueueFactory`].
//! Implementation types are returned directly — consumers receive concrete types
//! from the factory methods below and may use them as `impl Trait` at call sites.
//!
//! The `MessageBroker` contract (trait + value types + `MessageBrokerConfig`)
//! lives in `swe-edge-message-broker`; this crate owns the concrete backends and
//! the [`MessageBrokerFactory::from_config`] construction factory.

#[cfg(feature = "tokio-rt")]
use crate::api::broker::types::in_memory_message_broker::InMemoryMessageBroker;
use crate::api::broker::BrokerError;
use crate::api::broker::MessageBroker;
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
#[cfg(feature = "tokio-rt")]
use std::collections::HashMap;
#[cfg(feature = "tokio-rt")]
use std::sync::Arc;
use swe_edge_message_broker::{BackendKind, MessageBrokerConfig};
#[cfg(feature = "tokio-rt")]
use tokio::sync::RwLock;

impl MessageBrokerFactory {
    /// Return a [`swe_edge_configbuilder::ConfigBuilderImpl`] pre-seeded with this crate's package name and version.
    pub fn create_config_builder() -> swe_edge_configbuilder::ConfigBuilderImpl {
        let mut b = swe_edge_configbuilder::ConfigBuilderImpl::new();
        b = b.with_name(env!("CARGO_PKG_NAME"));
        b = b.with_version(env!("CARGO_PKG_VERSION"));
        b
    }

    /// Construct and wire a broker from a loaded [`MessageBrokerConfig`].
    ///
    /// The configuration vocabulary ([`MessageBrokerConfig`], [`BackendKind`]) is
    /// owned by the `swe-edge-message-broker` contract; this factory turns a
    /// validated config into a concrete backend instance.
    ///
    /// The backend is selected by [`MessageBrokerConfig::backend`]:
    /// - [`BackendKind::InMemory`] builds an in-process broadcast broker
    ///   (requires the `tokio-rt` feature).
    /// - [`BackendKind::Nats`] connects to the configured `url`
    ///   (requires the `nats` feature).
    ///
    /// # Errors
    ///
    /// - [`BrokerError::Unavailable`] if the requested backend's Cargo feature
    ///   is not compiled in.
    /// - [`BrokerError::Connection`] if a NATS connection cannot be established,
    ///   or if `backend = "nats"` was loaded without a `url`.
    pub async fn from_config(
        config: &MessageBrokerConfig,
    ) -> Result<Box<dyn MessageBroker>, BrokerError> {
        match config.backend {
            BackendKind::InMemory => {
                #[cfg(feature = "tokio-rt")]
                {
                    Ok(Box::new(Self::in_memory()) as Box<dyn MessageBroker>)
                }
                #[cfg(not(feature = "tokio-rt"))]
                {
                    Err(BrokerError::Unavailable(
                        "in_memory backend requires the `tokio-rt` feature".to_owned(),
                    ))
                }
            }
            BackendKind::Nats => {
                #[cfg(feature = "nats")]
                {
                    let url = config.url.as_deref().ok_or_else(|| {
                        BrokerError::Connection(
                            "nats backend requires a `url` but none was configured".to_owned(),
                        )
                    })?;
                    NatsMessageBroker::connect(url)
                        .await
                        .map(|b| Box::new(b) as Box<dyn MessageBroker>)
                }
                #[cfg(not(feature = "nats"))]
                {
                    Err(BrokerError::Unavailable(
                        "nats backend requires the `nats` feature".to_owned(),
                    ))
                }
            }
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
