//! Trait implementations for broker api types.

use futures::future::BoxFuture;

use crate::api::BrokerBackendConfig;
use crate::api::BrokerError;
use crate::api::BrokerProvider;
#[cfg(feature = "tokio-rt")]
use crate::api::InMemoryMessageBroker;
#[cfg(feature = "tokio-rt")]
use crate::api::Message;
#[cfg(feature = "tokio-rt")]
use crate::api::MessageBroker;
use crate::api::MessageBrokerFactory;
#[cfg(feature = "tokio-rt")]
use crate::api::MessageStream;

impl Default for BrokerBackendConfig {
    fn default() -> Self {
        Self {
            backend: "inmemory".into(),
            nats_url: "nats://localhost:4222".into(),
            kafka_brokers: "localhost:9092".into(),
        }
    }
}

impl BrokerProvider for MessageBrokerFactory {
    #[cfg(feature = "tokio-rt")]
    fn build_in_memory(&self) -> InMemoryMessageBroker {
        MessageBrokerFactory::in_memory()
    }

    fn build_from_config<'a>(
        &'a self,
        config: &'a swe_edge_message_broker::MessageBrokerConfig,
    ) -> BoxFuture<'a, Result<Box<dyn swe_edge_message_broker::MessageBroker>, BrokerError>> {
        Box::pin(MessageBrokerFactory::from_config(config))
    }
}

#[cfg(feature = "tokio-rt")]
impl InMemoryMessageBroker {
    fn check_topic(topic: &str) -> Result<(), BrokerError> {
        if topic.is_empty() {
            return Err(BrokerError::Publish {
                topic: topic.to_owned(),
                reason: "topic must not be empty".into(),
            });
        }
        if topic.len() > super::MAX_TOPIC_BYTES {
            return Err(BrokerError::Publish {
                topic: topic.to_owned(),
                reason: format!(
                    "topic exceeds maximum length of {} bytes",
                    super::MAX_TOPIC_BYTES
                ),
            });
        }
        Ok(())
    }
}

#[cfg(feature = "tokio-rt")]
impl MessageBroker for InMemoryMessageBroker {
    fn publish<'a>(
        &'a self,
        topic: &'a str,
        msg: Message,
    ) -> BoxFuture<'a, Result<(), BrokerError>> {
        use std::sync::Arc;
        let validation = Self::check_topic(topic);
        let topic = topic.to_owned();
        let channels = Arc::clone(&self.channels);
        Box::pin(async move {
            validation?;
            let map = channels.read().await;
            if let Some(tx) = map.get(&topic) {
                let _ = tx.send(msg);
            }
            Ok(())
        })
    }

    fn subscribe<'a>(
        &'a self,
        topic: &'a str,
    ) -> BoxFuture<'a, Result<MessageStream, BrokerError>> {
        use std::sync::Arc;
        use tokio::sync::broadcast;
        let topic = topic.to_owned();
        let channels = Arc::clone(&self.channels);
        Box::pin(async move {
            let rx = {
                let mut map = channels.write().await;
                let tx = map
                    .entry(topic.clone())
                    .or_insert_with(|| broadcast::channel(super::DEFAULT_CHANNEL_CAPACITY).0);
                tx.subscribe()
            };

            let stream = futures::stream::unfold(rx, |mut recv| async move {
                match recv.recv().await {
                    Ok(msg) => Some((Ok(msg), recv)),
                    Err(tokio::sync::broadcast::error::RecvError::Closed) => None,
                    Err(tokio::sync::broadcast::error::RecvError::Lagged(n)) => {
                        Some((Err(BrokerError::StreamLagged { count: n }), recv))
                    }
                }
            });

            Ok(Box::pin(stream) as MessageStream)
        })
    }

    fn health_check(&self) -> BoxFuture<'_, Result<(), BrokerError>> {
        Box::pin(async { Ok(()) })
    }
}
