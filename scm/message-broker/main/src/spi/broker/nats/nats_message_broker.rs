//! [`NatsMessageBroker`] — NATS-backed message broker via `async-nats`.

use std::collections::HashMap;

use bytes::Bytes;
use futures::future::BoxFuture;
use futures::StreamExt;

use crate::api::BrokerError;
use crate::api::Message;
use crate::api::MessageBroker;
use crate::api::MessageStream;

/// NATS-backed pub/sub broker.
///
/// Wraps an [`async_nats::Client`] to implement [`MessageBroker`].  Connect
/// via the `MessageBrokerFactory::nats` factory which handles the async handshake
/// and maps connection errors to [`BrokerError::Connection`].
///
/// Requires the `nats` feature.
pub(crate) struct NatsMessageBroker {
    client: async_nats::Client,
}

impl NatsMessageBroker {
    /// Establish a NATS connection and return a broker handle.
    pub(crate) async fn connect(url: impl Into<String>) -> Result<Self, BrokerError> {
        let client = async_nats::connect(url.into())
            .await
            .map_err(|e| BrokerError::Connection(e.to_string()))?;
        Ok(Self { client })
    }
}

impl MessageBroker for NatsMessageBroker {
    fn publish<'a>(
        &'a self,
        topic: &'a str,
        msg: Message,
    ) -> BoxFuture<'a, Result<(), BrokerError>> {
        let topic = topic.to_owned();
        let client = self.client.clone();
        Box::pin(async move {
            client
                .publish(topic.clone(), msg.payload)
                .await
                .map_err(|e| BrokerError::Publish {
                    topic,
                    reason: e.to_string(),
                })
        })
    }

    fn subscribe<'a>(
        &'a self,
        topic: &'a str,
    ) -> BoxFuture<'a, Result<MessageStream, BrokerError>> {
        let topic = topic.to_owned();
        let client = self.client.clone();
        Box::pin(async move {
            let subscriber =
                client
                    .subscribe(topic.clone())
                    .await
                    .map_err(|e| BrokerError::Subscribe {
                        topic,
                        reason: e.to_string(),
                    })?;

            let stream = subscriber.map(|nats_msg| {
                Ok(Message {
                    payload: Bytes::from(nats_msg.payload.to_vec()),
                    headers: HashMap::new(),
                })
            });

            Ok(Box::pin(stream) as MessageStream)
        })
    }

    fn health_check(&self) -> BoxFuture<'_, Result<(), BrokerError>> {
        let client = self.client.clone();
        Box::pin(async move {
            let _info = client.server_info();
            Ok(())
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nats_message_broker_is_send_and_sync() {
        fn _assert_send_sync<T: Send + Sync>() {}
        _assert_send_sync::<NatsMessageBroker>();
    }

    #[test]
    fn test_connect_returns_connection_error_for_unreachable_host() {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        let result = rt.block_on(NatsMessageBroker::connect("nats://127.0.0.1:4229"));
        // The Ok variant (NatsMessageBroker) is not Debug, so assert with a
        // static message rather than formatting `result`.
        assert!(
            matches!(result, Err(BrokerError::Connection(_))),
            "expected a Connection error from an unreachable NATS host"
        );
    }
}
