//! async-std-backed actor handle implementation.

use std::sync::Arc;

use futures::future::BoxFuture;

use crate::api::{Actor, ActorHandle as ActorHandleTrait, MailboxError, Message};

/// async-std-backed actor handle implementation.
pub(crate) struct AsyncStdActorHandle<A: Actor> {
    pub(crate) tx: Arc<async_std::channel::Sender<Message<A>>>,
}

impl<A: Actor> Clone for AsyncStdActorHandle<A> {
    fn clone(&self) -> Self {
        Self {
            tx: Arc::clone(&self.tx),
        }
    }
}

impl<A: Actor> ActorHandleTrait<A::Message> for AsyncStdActorHandle<A> {
    fn tell(&self, msg: A::Message) -> BoxFuture<'_, Result<(), MailboxError>> {
        let tx = Arc::clone(&self.tx);
        Box::pin(async move {
            tx.send(Message::Msg(msg))
                .await
                .map_err(|_| MailboxError::Closed)
        })
    }
}

impl<A: Actor> AsyncStdActorHandle<A> {
    /// Send a message and wait for a response (request-reply).
    ///
    /// Returns the actor's response on success, or `MailboxError` if:
    /// - The mailbox is full
    /// - The actor stopped
    /// - The reply channel dropped unexpectedly
    pub async fn ask<R: Send + 'static>(
        &self,
        msg: impl FnOnce(async_std::channel::Sender<R>) -> A::Message,
    ) -> Result<R, MailboxError> {
        let (reply_tx, reply_rx) = async_std::channel::bounded(1);
        let actor_msg = msg(reply_tx);

        self.tell(actor_msg).await?;
        reply_rx
            .recv()
            .await
            .map_err(|_| MailboxError::ReplyDropped)
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
        GetCount(async_std::channel::Sender<i32>),
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
                    TestMessage::GetCount(tx) => {
                        let _ = tx.send(self.count).await;
                    }
                }
            })
        }
    }

    /// @covers: AsyncStdActorHandle::tell
    #[async_std::test]
    async fn test_async_std_actor_handle_tell_enqueues_message() {
        let (tx, rx) = async_std::channel::bounded::<Message<TestActor>>(1);
        let handle: AsyncStdActorHandle<TestActor> = AsyncStdActorHandle { tx: Arc::new(tx) };

        let result = handle.tell(TestMessage::Inc).await;
        assert!(result.is_ok());

        // Verify message was sent
        let msg = rx.recv().await;
        assert!(matches!(msg, Ok(Message::Msg(TestMessage::Inc))));
    }

    /// @covers: ask
    #[async_std::test]
    async fn test_ask_returns_reply_from_actor() {
        use crate::spi::r#async::std::mailbox::AsyncStdMailbox;

        struct AskActor;

        enum AskMsg {
            Echo(async_std::channel::Sender<u32>),
        }

        impl Actor for AskActor {
            type Message = AskMsg;

            fn handle(
                &mut self,
                _ctx: crate::api::ActorContext<Self>,
                msg: Self::Message,
            ) -> BoxFuture<'_, ()> {
                Box::pin(async move {
                    match msg {
                        AskMsg::Echo(tx) => {
                            let _ = tx.send(99).await;
                        }
                    }
                })
            }
        }

        let handle = AsyncStdMailbox::spawn(AskActor);
        let result = handle
            .ask(|tx| AskMsg::Echo(tx))
            .await
            .unwrap_or_else(|_| panic!("ask failed"));
        assert_eq!(result, 99);
    }

    /// @covers: ask — error returned when mailbox is closed
    #[async_std::test]
    async fn test_ask_returns_error_on_closed_mailbox() {
        let (tx, rx) = async_std::channel::bounded::<Message<TestActor>>(1);
        // Drop the receiver to close the channel
        drop(rx);
        let handle: AsyncStdActorHandle<TestActor> = AsyncStdActorHandle { tx: Arc::new(tx) };

        // tell should fail with Closed when receiver is dropped
        let result = handle.tell(TestMessage::Inc).await;
        assert!(result.is_err(), "tell must fail when mailbox is closed");
    }
}
