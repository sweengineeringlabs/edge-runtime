//! Tokio-backed actor mailbox — message loop and spawn.

use tokio::sync::mpsc;

use crate::api::{Actor, ActorContext, Message};

use super::actor_handle::TokioActorHandle;
use super::stop_handle::TokioStopHandle;

/// Bounded channel capacity for actor mailboxes.
pub(crate) const MAILBOX_CAPACITY: usize = 1024;

/// Tokio-backed actor mailbox — manages message channel and actor task lifecycle.
pub(crate) struct TokioMailbox;

impl TokioMailbox {
    /// Spawn a tokio actor and return a handle to send messages.
    ///
    /// The actor runs in a spawned tokio task, processing messages sequentially.
    /// Dropping the handle will stop accepting new messages, but the actor will
    /// continue processing until the current message finishes.
    #[expect(dead_code, reason = "SEA spi/ anchor — used when SAF spawn is wired in")]
    pub(crate) fn spawn<A: Actor>(actor: A) -> TokioActorHandle<A> {
        let (tx, rx) = mpsc::channel(MAILBOX_CAPACITY);
        let tx = std::sync::Arc::new(tx);

        tokio::spawn(Self::run_actor_loop(actor, rx));

        TokioActorHandle { tx }
    }

    /// Spawn a tokio actor with lifecycle management (stop handle).
    ///
    /// Returns both a message handle and a stop handle. Calling `stop()` on the
    /// handle will signal the actor to shut down gracefully.
    #[expect(dead_code, reason = "SEA spi/ anchor — used when SAF spawn is wired in")]
    pub(crate) fn spawn_with_stop<A: Actor>(actor: A) -> (TokioActorHandle<A>, TokioStopHandle<A>) {
        let (tx, rx) = mpsc::channel(MAILBOX_CAPACITY);
        let tx = std::sync::Arc::new(tx);

        tokio::spawn(Self::run_actor_loop(actor, rx));

        let handle = TokioActorHandle {
            tx: std::sync::Arc::clone(&tx),
        };
        let stop = TokioStopHandle { tx };

        (handle, stop)
    }

    /// The actor's main message processing loop.
    pub(crate) async fn run_actor_loop<A: Actor>(mut actor: A, mut rx: mpsc::Receiver<Message<A>>) {
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::future::BoxFuture;

    use crate::api::{ActorHandle, StopHandle};

    struct TokioMailboxTestActor {
        count: i32,
    }

    enum TokioMailboxTestMessage {
        Inc,
        GetCount(tokio::sync::oneshot::Sender<i32>),
    }

    impl Actor for TokioMailboxTestActor {
        type Message = TokioMailboxTestMessage;

        fn handle(&mut self, _ctx: ActorContext<Self>, msg: Self::Message) -> BoxFuture<'_, ()> {
            Box::pin(async move {
                match msg {
                    TokioMailboxTestMessage::Inc => self.count += 1,
                    TokioMailboxTestMessage::GetCount(tx) => {
                        let _ = tx.send(self.count);
                    }
                }
            })
        }
    }

    /// @covers: spawn
    #[test]
    fn test_spawn_returns_handle_that_delivers_messages() {
        let rt =
            tokio::runtime::Runtime::new().unwrap_or_else(|e| panic!("runtime init failed: {e}"));
        rt.block_on(async {
            let actor = TokioMailboxTestActor { count: 0 };
            let handle = TokioMailbox::spawn(actor);

            handle
                .tell(TokioMailboxTestMessage::Inc)
                .await
                .unwrap_or_else(|_| panic!("tell failed"));

            let (tx, rx) = tokio::sync::oneshot::channel();
            handle
                .tell(TokioMailboxTestMessage::GetCount(tx))
                .await
                .unwrap_or_else(|_| panic!("tell failed"));
            let count = rx.await.unwrap_or_else(|_| panic!("recv failed"));
            assert_eq!(count, 1);
        });
    }

    /// @covers: spawn_with_stop
    #[test]
    fn test_spawn_with_stop_stop_handle_closes_mailbox() {
        let rt =
            tokio::runtime::Runtime::new().unwrap_or_else(|e| panic!("runtime init failed: {e}"));
        rt.block_on(async {
            let actor = TokioMailboxTestActor { count: 0 };
            let (handle, stop) = TokioMailbox::spawn_with_stop(actor);

            stop.stop().await;
            tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;

            let result = handle.tell(TokioMailboxTestMessage::Inc).await;
            assert!(result.is_err(), "mailbox must be closed after stop");
        });
    }

    /// @covers: run_actor_loop
    #[test]
    fn test_run_actor_loop_processes_messages_sequentially() {
        let rt =
            tokio::runtime::Runtime::new().unwrap_or_else(|e| panic!("runtime init failed: {e}"));
        rt.block_on(async {
            let actor = TokioMailboxTestActor { count: 0 };
            let handle = TokioMailbox::spawn(actor);

            for _ in 0..5 {
                handle
                    .tell(TokioMailboxTestMessage::Inc)
                    .await
                    .unwrap_or_else(|_| panic!("tell failed"));
            }

            let (tx, rx) = tokio::sync::oneshot::channel();
            handle
                .tell(TokioMailboxTestMessage::GetCount(tx))
                .await
                .unwrap_or_else(|_| panic!("tell failed"));
            let count = rx.await.unwrap_or_else(|_| panic!("recv failed"));
            assert_eq!(count, 5, "all five increments must be processed in order");
        });
    }

    /// @covers: spawn
    #[tokio::test]
    async fn test_spawn_processes_messages() {
        let actor = TokioMailboxTestActor { count: 0 };
        let handle = TokioMailbox::spawn(actor);

        handle
            .tell(TokioMailboxTestMessage::Inc)
            .await
            .unwrap_or_else(|_| panic!("tell failed"));
        handle
            .tell(TokioMailboxTestMessage::Inc)
            .await
            .unwrap_or_else(|_| panic!("tell failed"));

        let (tx, rx) = tokio::sync::oneshot::channel();
        handle
            .tell(TokioMailboxTestMessage::GetCount(tx))
            .await
            .unwrap_or_else(|_| panic!("tell failed"));
        let count = rx.await.unwrap_or_else(|_| panic!("recv failed"));
        assert_eq!(count, 2);
    }

    /// @covers: spawn_with_stop
    #[tokio::test]
    async fn test_spawn_with_stop_graceful_shutdown() {
        let actor = TokioMailboxTestActor { count: 0 };
        let (handle, stop) = TokioMailbox::spawn_with_stop(actor);

        handle
            .tell(TokioMailboxTestMessage::Inc)
            .await
            .unwrap_or_else(|_| panic!("tell failed"));
        stop.stop().await;

        // Give actor loop time to process Stop signal
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;

        let result = handle.tell(TokioMailboxTestMessage::Inc).await;
        assert!(result.is_err(), "should not accept messages after stop");
    }

    /// @covers: run_actor_loop
    #[tokio::test]
    async fn test_sequential_message_processing() {
        let actor = TokioMailboxTestActor { count: 0 };
        let handle = TokioMailbox::spawn(actor);

        for _ in 0..100 {
            handle
                .tell(TokioMailboxTestMessage::Inc)
                .await
                .unwrap_or_else(|_| panic!("tell failed"));
        }

        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

        let (tx, rx) = tokio::sync::oneshot::channel();
        handle
            .tell(TokioMailboxTestMessage::GetCount(tx))
            .await
            .unwrap_or_else(|_| panic!("tell failed"));
        let count = rx.await.unwrap_or_else(|_| panic!("recv failed"));
        assert_eq!(count, 100, "all messages should be processed sequentially");
    }
}
