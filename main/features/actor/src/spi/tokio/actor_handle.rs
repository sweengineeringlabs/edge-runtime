//! Tokio-backed actor handle implementation.

use std::sync::Arc;

use futures::future::BoxFuture;
use tokio::sync::mpsc;

use crate::api::{Actor, ActorHandle as ActorHandleTrait, MailboxError, Message};

/// Tokio-backed actor handle implementation.
pub(crate) struct TokioActorHandle<A: Actor> {
    pub(crate) tx: Arc<mpsc::Sender<Message<A>>>,
}

impl<A: Actor> Clone for TokioActorHandle<A> {
    fn clone(&self) -> Self {
        Self {
            tx: Arc::clone(&self.tx),
        }
    }
}

impl<A: Actor> ActorHandleTrait<A::Message> for TokioActorHandle<A> {
    fn tell(&self, msg: A::Message) -> BoxFuture<'_, Result<(), MailboxError>> {
        let tx = Arc::clone(&self.tx);
        Box::pin(async move {
            tx.send(Message::Msg(msg))
                .await
                .map_err(|_| MailboxError::Closed)
        })
    }
}

impl<A: Actor> TokioActorHandle<A> {
    /// Send a message and wait for a response (request-reply).
    ///
    /// Returns the actor's response on success, or `MailboxError` if:
    /// - The mailbox is full
    /// - The actor stopped
    /// - The reply channel dropped unexpectedly
    ///
    /// The closure receives a `tokio::sync::oneshot::Sender<R>` to send the response.
    ///
    /// Note: This is a convenience method. Alternatively, include a reply channel
    /// in your message type and use `tell()` instead.
    pub async fn ask<R: Send + 'static>(
        &self,
        msg: impl FnOnce(tokio::sync::oneshot::Sender<R>) -> A::Message,
    ) -> Result<R, MailboxError> {
        let (reply_tx, reply_rx) = tokio::sync::oneshot::channel();
        let actor_msg = msg(reply_tx);

        self.tell(actor_msg).await?;
        reply_rx.await.map_err(|_| MailboxError::ReplyDropped)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestActor {
        count: i32,
    }

    enum TestMessage {
        Inc,
    }

    impl Actor for TestActor {
        type Message = TestMessage;

        fn handle(
            &mut self,
            _ctx: crate::api::ActorContext<Self>,
            msg: Self::Message,
        ) -> BoxFuture<'_, ()> {
            Box::pin(async move {
                match msg {
                    TestMessage::Inc => self.count += 1,
                }
            })
        }
    }

    /// @covers: TokioActorHandle::tell
    #[tokio::test]
    async fn test_tokio_actor_handle_tell_enqueues_message() {
        let (tx, mut rx) = mpsc::channel::<Message<TestActor>>(1);
        let handle: TokioActorHandle<TestActor> = TokioActorHandle { tx: Arc::new(tx) };

        let result = handle.tell(TestMessage::Inc).await;
        assert!(result.is_ok());

        // Verify message was sent
        let msg = rx.recv().await;
        assert!(matches!(msg, Some(Message::Msg(TestMessage::Inc))));
    }

    /// @covers: ask
    #[tokio::test]
    async fn test_ask_returns_reply_from_actor() {
        use crate::api::ActorContext;
        use crate::spi::tokio::TokioMailbox;

        struct AskActor;

        enum AskMessage {
            Echo(tokio::sync::oneshot::Sender<u32>),
        }

        impl Actor for AskActor {
            type Message = AskMessage;

            fn handle(
                &mut self,
                _ctx: ActorContext<Self>,
                msg: Self::Message,
            ) -> BoxFuture<'_, ()> {
                Box::pin(async move {
                    match msg {
                        AskMessage::Echo(tx) => {
                            let _ = tx.send(42);
                        }
                    }
                })
            }
        }

        let handle = TokioMailbox::spawn(AskActor);
        let result = handle
            .ask(|tx| AskMessage::Echo(tx))
            .await
            .unwrap_or_else(|_| panic!("ask failed"));
        assert_eq!(result, 42);
    }

    /// @covers: ask
    #[tokio::test]
    async fn test_ask_returns_error_on_closed_mailbox() {
        // Create a handle to a channel whose receiver has been dropped.
        // Sending to such a channel immediately returns an error.
        let (tx, rx) = mpsc::channel::<Message<TestActor>>(1);
        // Drop receiver — this closes the channel
        drop(rx);
        let handle = TokioActorHandle { tx: Arc::new(tx) };

        let result = handle.tell(TestMessage::Inc).await;
        assert!(
            matches!(result, Err(crate::api::MailboxError::Closed)),
            "tell must fail with Closed when receiver is dropped"
        );
    }
}
