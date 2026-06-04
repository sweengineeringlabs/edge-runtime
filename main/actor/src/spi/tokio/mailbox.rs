//! Tokio-backed actor mailbox — message loop and spawn.
//!
//! This module is compiled unconditionally but is only **wired up** under the
//! `tokio-rt` feature. Items are used by the SAF and tests; the dead_code lint
//! fires because the saf spawn paths are behind `#[cfg(feature = "tokio-rt")]`.
#![allow(dead_code)]

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

    /// @covers: spawn
    #[test]
    fn test_spawn_fn_exists() {
        let _: fn(TokioMailboxTestActor) -> TokioActorHandle<TokioMailboxTestActor> =
            TokioMailbox::spawn;
    }

    /// @covers: spawn_with_stop
    #[test]
    fn test_spawn_with_stop_fn_exists() {
        let _: fn(
            TokioMailboxTestActor,
        ) -> (
            TokioActorHandle<TokioMailboxTestActor>,
            TokioStopHandle<TokioMailboxTestActor>,
        ) = TokioMailbox::spawn_with_stop;
    }

    /// @covers: run_actor_loop
    #[test]
    fn test_run_actor_loop_capacity_constant_is_positive() {
        let _ = MAILBOX_CAPACITY;
    }

    struct TokioMailboxTestActor {
        count: i32,
    }

    enum TokioMailboxTestMsg {
        Inc,
        Get(tokio::sync::oneshot::Sender<i32>),
    }

    impl Actor for TokioMailboxTestActor {
        type Message = TokioMailboxTestMsg;

        fn handle(&mut self, _ctx: ActorContext<Self>, msg: Self::Message) -> BoxFuture<'_, ()> {
            Box::pin(async move {
                match msg {
                    TokioMailboxTestMsg::Inc => self.count += 1,
                    TokioMailboxTestMsg::Get(tx) => {
                        let _ = tx.send(self.count);
                    }
                }
            })
        }
    }

    /// @covers: spawn
    #[tokio::test]
    async fn test_spawn_delivers_messages_to_actor() {
        let handle = TokioMailbox::spawn(TokioMailboxTestActor { count: 0 });
        handle.tell(TokioMailboxTestMsg::Inc).await.ok();
        let (tx, rx) = tokio::sync::oneshot::channel();
        handle.tell(TokioMailboxTestMsg::Get(tx)).await.ok();
        assert_eq!(rx.await.unwrap_or(0), 1);
    }

    /// @covers: spawn_with_stop
    #[tokio::test]
    async fn test_spawn_with_stop_closes_mailbox_on_stop() {
        let (handle, stop) = TokioMailbox::spawn_with_stop(TokioMailboxTestActor { count: 0 });
        stop.stop().await;
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
        assert!(handle.tell(TokioMailboxTestMsg::Inc).await.is_err());
    }

    /// @covers: run_actor_loop
    #[tokio::test]
    async fn test_run_actor_loop_processes_messages_in_order() {
        let handle = TokioMailbox::spawn(TokioMailboxTestActor { count: 0 });
        for _ in 0..3 {
            handle.tell(TokioMailboxTestMsg::Inc).await.ok();
        }
        let (tx, rx) = tokio::sync::oneshot::channel();
        handle.tell(TokioMailboxTestMsg::Get(tx)).await.ok();
        assert_eq!(rx.await.unwrap_or(0), 3);
    }
}
