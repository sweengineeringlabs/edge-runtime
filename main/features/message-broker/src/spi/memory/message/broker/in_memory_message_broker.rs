//! [`InMemoryMessageBroker`] — tokio broadcast-channel backed broker.

use std::collections::HashMap;
use std::sync::Arc;

use futures::future::BoxFuture;
use tokio::sync::{broadcast, RwLock};

use crate::api::broker::broker_error::BrokerError;
use crate::api::broker::Message;
use crate::api::broker::MessageBroker;
use crate::api::broker::MessageStream;
use crate::api::traits::Validator;

/// Capacity of each topic's broadcast channel — inherited from core configuration.
const CHANNEL_CAPACITY: usize = crate::core::DEFAULT_CHANNEL_CAPACITY;

/// In-memory pub/sub broker backed by [`tokio::sync::broadcast`].
///
/// Topics are created lazily on first subscription.  Multiple handles to the
/// same broker share a single channel map via an internal `Arc`, so cloning
/// this struct produces another handle to the same broker.
///
/// Requires the `tokio-rt` feature.
#[derive(Clone)]
pub(crate) struct InMemoryMessageBroker {
    channels: Arc<RwLock<HashMap<String, broadcast::Sender<Message>>>>,
}

impl InMemoryMessageBroker {
    pub(crate) fn new() -> Self {
        Self {
            channels: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Validate that a topic string meets broker constraints.
    fn check_topic(topic: &str) -> Result<(), BrokerError> {
        if topic.is_empty() {
            return Err(BrokerError::Publish {
                topic: topic.to_owned(),
                reason: "topic must not be empty".into(),
            });
        }
        if topic.len() > crate::core::broker::MAX_TOPIC_BYTES {
            return Err(BrokerError::Publish {
                topic: topic.to_owned(),
                reason: format!(
                    "topic exceeds maximum length of {} bytes",
                    crate::core::broker::MAX_TOPIC_BYTES
                ),
            });
        }
        Ok(())
    }
}

impl Validator for InMemoryMessageBroker {
    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

impl MessageBroker for InMemoryMessageBroker {
    fn publish<'a>(
        &'a self,
        topic: &'a str,
        msg: Message,
    ) -> BoxFuture<'a, Result<(), BrokerError>> {
        let validation = Self::check_topic(topic);
        let topic = topic.to_owned();
        let channels = Arc::clone(&self.channels);
        Box::pin(async move {
            validation?;
            let map = channels.read().await;
            if let Some(tx) = map.get(&topic) {
                // SendError means no active receivers — silently drop (fire-and-forget).
                let _ = tx.send(msg);
            }
            Ok(())
        })
    }

    fn subscribe<'a>(
        &'a self,
        topic: &'a str,
    ) -> BoxFuture<'a, Result<MessageStream, BrokerError>> {
        let topic = topic.to_owned();
        let channels = Arc::clone(&self.channels);
        Box::pin(async move {
            let rx = {
                let mut map = channels.write().await;
                let tx = map
                    .entry(topic.clone())
                    .or_insert_with(|| broadcast::channel(CHANNEL_CAPACITY).0);
                tx.subscribe()
            };

            let stream = futures::stream::unfold(rx, |mut recv| async move {
                match recv.recv().await {
                    Ok(msg) => Some((Ok(msg), recv)),
                    Err(broadcast::error::RecvError::Closed) => None,
                    Err(broadcast::error::RecvError::Lagged(n)) => {
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

#[cfg(test)]
mod tests {
    use super::*;
    use bytes::Bytes;
    use futures::StreamExt as _;

    #[test]
    fn test_new_creates_empty_broker() {
        let broker = InMemoryMessageBroker::new();
        drop(broker);
    }

    #[test]
    fn test_validate_returns_ok() {
        assert!(InMemoryMessageBroker::new().validate().is_ok());
    }

    #[tokio::test]
    async fn test_subscribe_then_publish_delivers_message() {
        let broker = InMemoryMessageBroker::new();
        let mut stream = broker.subscribe("events").await.unwrap();
        broker
            .publish("events", Message::new(b"hello".as_ref()))
            .await
            .unwrap();
        let msg = stream.next().await.unwrap().unwrap();
        assert_eq!(msg.payload, Bytes::from_static(b"hello"));
    }

    #[tokio::test]
    async fn test_publish_before_subscribe_drops_message() {
        let broker = InMemoryMessageBroker::new();
        broker
            .publish("void", Message::new(b"dropped".as_ref()))
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_multiple_subscribers_each_receive_message() {
        let broker = InMemoryMessageBroker::new();
        let mut s1 = broker.subscribe("topic").await.unwrap();
        let mut s2 = broker.subscribe("topic").await.unwrap();
        broker
            .publish("topic", Message::new(b"fanout".as_ref()))
            .await
            .unwrap();
        let m1 = s1.next().await.unwrap().unwrap();
        let m2 = s2.next().await.unwrap().unwrap();
        assert_eq!(m1.payload, m2.payload);
    }

    #[tokio::test]
    async fn test_health_check_returns_ok() {
        let broker = InMemoryMessageBroker::new();
        assert!(broker.health_check().await.is_ok());
    }

    #[tokio::test]
    async fn test_publish_to_different_topics_does_not_cross_deliver() {
        let broker = InMemoryMessageBroker::new();
        let mut orders = broker.subscribe("orders").await.unwrap();
        broker
            .publish("payments", Message::new(b"pay".as_ref()))
            .await
            .unwrap();
        broker
            .publish("orders", Message::new(b"order".as_ref()))
            .await
            .unwrap();
        let msg = orders.next().await.unwrap().unwrap();
        assert_eq!(msg.payload, Bytes::from_static(b"order"));
    }

    #[tokio::test]
    async fn test_clone_shares_channels_with_original() {
        let broker = InMemoryMessageBroker::new();
        let clone = broker.clone();
        let mut stream = broker.subscribe("shared").await.unwrap();
        clone
            .publish("shared", Message::new(b"via-clone".as_ref()))
            .await
            .unwrap();
        let msg = stream.next().await.unwrap().unwrap();
        assert_eq!(msg.payload, Bytes::from_static(b"via-clone"));
    }

    #[test]
    fn test_check_topic_empty_returns_error() {
        assert!(InMemoryMessageBroker::check_topic("").is_err());
    }

    #[test]
    fn test_check_topic_valid_returns_ok() {
        assert!(InMemoryMessageBroker::check_topic("events").is_ok());
    }

    #[test]
    fn test_check_topic_too_long_returns_error() {
        let long_topic = "a".repeat(crate::core::broker::MAX_TOPIC_BYTES + 1);
        assert!(InMemoryMessageBroker::check_topic(&long_topic).is_err());
    }
}
