//! [`TokioActor`] — tokio-backed concurrent message loop.

use std::sync::Arc;

use futures::future::BoxFuture;
use tokio::sync::mpsc;

use crate::api::{Actor, ActorContext, ActorHandle as ActorHandleTrait, MailboxError, StopHandle};

/// Bounded channel capacity for actor mailboxes.
const MAILBOX_CAPACITY: usize = 1024;

/// Message wrapper for the actor's internal queue.
enum Message<A: Actor> {
    /// User-sent message to the actor.
    Msg(A::Message),
    /// Signal to stop the actor.
    Stop,
}

/// Tokio-backed actor handle implementation.
pub(crate) struct TokioActorHandle<A: Actor> {
    tx: Arc<mpsc::Sender<Message<A>>>,
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
    #[allow(dead_code)]
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

/// Tokio-backed stop handle implementation.
pub(crate) struct TokioStopHandle<A: Actor> {
    tx: Arc<mpsc::Sender<Message<A>>>,
}

impl<A: Actor> Clone for TokioStopHandle<A> {
    fn clone(&self) -> Self {
        Self {
            tx: Arc::clone(&self.tx),
        }
    }
}

impl<A: Actor> StopHandle for TokioStopHandle<A> {
    fn stop(&self) -> BoxFuture<'_, ()> {
        let tx = Arc::clone(&self.tx);
        Box::pin(async move {
            let _ = tx.send(Message::Stop).await;
        })
    }
}

/// Spawn a tokio actor and return a handle to send messages.
///
/// The actor runs in a spawned tokio task, processing messages sequentially.
/// Dropping the handle will stop accepting new messages, but the actor will
/// continue processing until the current message finishes.
pub(crate) fn spawn_tokio_actor<A: Actor>(actor: A) -> TokioActorHandle<A> {
    let (tx, rx) = mpsc::channel(MAILBOX_CAPACITY);
    let tx = Arc::new(tx);

    tokio::spawn(run_actor_loop(actor, rx));

    TokioActorHandle { tx }
}

/// Spawn a tokio actor with lifecycle management (stop handle).
///
/// Returns both a message handle and a stop handle. Calling `stop()` on the
/// handle will signal the actor to shut down gracefully.
pub(crate) fn spawn_tokio_actor_with_stop<A: Actor>(
    actor: A,
) -> (TokioActorHandle<A>, TokioStopHandle<A>) {
    let (tx, rx) = mpsc::channel(MAILBOX_CAPACITY);
    let tx = Arc::new(tx);

    tokio::spawn(run_actor_loop(actor, rx));

    let handle = TokioActorHandle {
        tx: Arc::clone(&tx),
    };
    let stop = TokioStopHandle { tx };

    (handle, stop)
}

/// The actor's main message processing loop.
async fn run_actor_loop<A: Actor>(mut actor: A, mut rx: mpsc::Receiver<Message<A>>) {
    let ctx = ActorContext::new();

    while let Some(msg) = rx.recv().await {
        match msg {
            Message::Msg(m) => {
                actor.handle(ctx.clone(), m).await;
            }
            Message::Stop => {
                actor.on_stop().await;
                break;
            }
        }
    }

    // If receiver dropped without Stop signal, run on_stop anyway
    actor.on_stop().await;
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestActor {
        count: i32,
    }

    enum TestMessage {
        Inc,
        GetCount(tokio::sync::oneshot::Sender<i32>),
    }

    impl Actor for TestActor {
        type Message = TestMessage;

        fn handle(&mut self, _ctx: ActorContext<Self>, msg: Self::Message) -> BoxFuture<'_, ()> {
            Box::pin(async move {
                match msg {
                    TestMessage::Inc => self.count += 1,
                    TestMessage::GetCount(tx) => {
                        let _ = tx.send(self.count);
                    }
                }
            })
        }
    }

    /// @covers: spawn_tokio_actor
    #[tokio::test]
    async fn test_spawn_tokio_actor_processes_messages() {
        let actor = TestActor { count: 0 };
        let handle = spawn_tokio_actor(actor);

        handle.tell(TestMessage::Inc).await.unwrap();
        handle.tell(TestMessage::Inc).await.unwrap();

        let (tx, rx) = tokio::sync::oneshot::channel();
        handle.tell(TestMessage::GetCount(tx)).await.unwrap();
        let count = rx.await.unwrap();
        assert_eq!(count, 2);
    }

    /// @covers: spawn_tokio_actor_with_stop
    #[tokio::test]
    async fn test_spawn_tokio_actor_with_stop_graceful_shutdown() {
        let actor = TestActor { count: 0 };
        let (handle, stop) = spawn_tokio_actor_with_stop(actor);

        handle.tell(TestMessage::Inc).await.unwrap();
        stop.stop().await;

        // Give actor loop time to process Stop signal
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;

        let result = handle.tell(TestMessage::Inc).await;
        assert!(result.is_err(), "should not accept messages after stop");
    }

    /// @covers: run_actor_loop
    #[tokio::test]
    async fn test_sequential_message_processing() {
        let actor = TestActor { count: 0 };
        let handle = spawn_tokio_actor(actor);

        for _ in 0..100 {
            handle.tell(TestMessage::Inc).await.unwrap();
        }

        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

        let (tx, rx) = tokio::sync::oneshot::channel();
        handle.tell(TestMessage::GetCount(tx)).await.unwrap();
        let count = rx.await.unwrap();
        assert_eq!(count, 100, "all messages should be processed sequentially");
    }
}
