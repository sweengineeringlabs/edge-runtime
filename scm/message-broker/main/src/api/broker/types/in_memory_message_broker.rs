//! [`InMemoryMessageBroker`] — tokio broadcast-channel backed broker.

use std::collections::HashMap;
use std::sync::Arc;

use futures::future::BoxFuture;
use tokio::sync::{broadcast, RwLock};

use crate::api::broker::BrokerError;
use crate::api::broker::Message;
use crate::api::broker::MessageBroker;
use crate::api::broker::MessageStream;
use crate::api::traits::Validator;

/// Capacity of each topic's broadcast channel.
const CHANNEL_CAPACITY: usize = 1024;

/// Maximum topic name length in bytes enforced on publish and subscribe calls.
const MAX_TOPIC_BYTES: usize = 256;

/// In-memory pub/sub broker backed by [`tokio::sync::broadcast`].
///
/// Topics are created lazily on first subscription.  Multiple handles to the
/// same broker share a single channel map via an internal `Arc`, so cloning
/// this struct produces another handle to the same broker.
///
/// Requires the `tokio-rt` feature.
#[derive(Clone)]
pub struct InMemoryMessageBroker {
    pub(crate) channels: Arc<RwLock<HashMap<String, broadcast::Sender<Message>>>>,
}

impl InMemoryMessageBroker {
    /// Validate that a topic string meets broker constraints.
    fn check_topic(topic: &str) -> Result<(), BrokerError> {
        if topic.is_empty() {
            return Err(BrokerError::Publish {
                topic: topic.to_owned(),
                reason: "topic must not be empty".into(),
            });
        }
        if topic.len() > MAX_TOPIC_BYTES {
            return Err(BrokerError::Publish {
                topic: topic.to_owned(),
                reason: format!("topic exceeds maximum length of {} bytes", MAX_TOPIC_BYTES),
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
