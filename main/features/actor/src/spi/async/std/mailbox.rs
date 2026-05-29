//! async-std-backed actor mailbox — message loop and spawn.

use crate::api::{Actor, ActorContext, Message};

use super::actor_handle::AsyncStdActorHandle;
use super::stop_handle::AsyncStdStopHandle;

/// Bounded channel capacity for actor mailboxes.
pub(crate) const MAILBOX_CAPACITY: usize = 1024;

/// async-std-backed actor mailbox — manages message channel and actor task lifecycle.
pub(crate) struct AsyncStdMailbox;

impl AsyncStdMailbox {
    /// Spawn an async-std actor and return a handle to send messages.
    ///
    /// The actor runs in a spawned async-std task, processing messages sequentially.
    pub(crate) fn spawn<A: Actor>(actor: A) -> AsyncStdActorHandle<A> {
        let (tx, rx) = async_std::channel::bounded(MAILBOX_CAPACITY);
        let tx = std::sync::Arc::new(tx);

        async_std::task::spawn(Self::run_actor_loop(actor, rx));

        AsyncStdActorHandle { tx }
    }

    /// Spawn an async-std actor with lifecycle management (stop handle).
    pub(crate) fn spawn_with_stop<A: Actor>(
        actor: A,
    ) -> (AsyncStdActorHandle<A>, AsyncStdStopHandle<A>) {
        let (tx, rx) = async_std::channel::bounded(MAILBOX_CAPACITY);
        let tx = std::sync::Arc::new(tx);

        async_std::task::spawn(Self::run_actor_loop(actor, rx));

        let handle = AsyncStdActorHandle {
            tx: std::sync::Arc::clone(&tx),
        };
        let stop = AsyncStdStopHandle { tx };

        (handle, stop)
    }

    /// The actor's main message processing loop.
    pub(crate) async fn run_actor_loop<A: Actor>(
        mut actor: A,
        rx: async_std::channel::Receiver<Message<A>>,
    ) {
        let ctx = ActorContext::new();

        while let Ok(msg) = rx.recv().await {
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

    struct TestActor {
        count: i32,
    }

    enum TestMessage {
        Inc,
        GetCount(async_std::channel::Sender<i32>),
    }

    impl Actor for TestActor {
        type Message = TestMessage;

        fn handle(&mut self, _ctx: ActorContext<Self>, msg: Self::Message) -> BoxFuture<'_, ()> {
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

    /// @covers: AsyncStdMailbox::spawn
    #[async_std::test]
    async fn test_spawn_processes_messages() {
        let actor = TestActor { count: 0 };
        let handle = AsyncStdMailbox::spawn(actor);

        handle
            .tell(TestMessage::Inc)
            .await
            .unwrap_or_else(|_| panic!("tell failed"));
        handle
            .tell(TestMessage::Inc)
            .await
            .unwrap_or_else(|_| panic!("tell failed"));

        let (tx, rx) = async_std::channel::bounded(1);
        handle
            .tell(TestMessage::GetCount(tx))
            .await
            .unwrap_or_else(|_| panic!("tell failed"));
        let count = rx.recv().await.unwrap_or_else(|_| panic!("recv failed"));
        assert_eq!(count, 2);
    }

    /// @covers: AsyncStdMailbox::spawn_with_stop
    #[async_std::test]
    async fn test_spawn_with_stop_graceful_shutdown() {
        let actor = TestActor { count: 0 };
        let (handle, stop) = AsyncStdMailbox::spawn_with_stop(actor);

        handle
            .tell(TestMessage::Inc)
            .await
            .unwrap_or_else(|_| panic!("tell failed"));
        stop.stop().await;

        // Give actor loop time to process Stop signal
        async_std::task::sleep(std::time::Duration::from_millis(10)).await;

        let result = handle.tell(TestMessage::Inc).await;
        assert!(result.is_err(), "should not accept messages after stop");
    }

    /// @covers: AsyncStdMailbox::run_actor_loop
    #[async_std::test]
    async fn test_sequential_message_processing() {
        let actor = TestActor { count: 0 };
        let handle = AsyncStdMailbox::spawn(actor);

        for _ in 0..100 {
            handle
                .tell(TestMessage::Inc)
                .await
                .unwrap_or_else(|_| panic!("tell failed"));
        }

        async_std::task::sleep(std::time::Duration::from_millis(50)).await;

        let (tx, rx) = async_std::channel::bounded(1);
        handle
            .tell(TestMessage::GetCount(tx))
            .await
            .unwrap_or_else(|_| panic!("tell failed"));
        let count = rx.recv().await.unwrap_or_else(|_| panic!("recv failed"));
        assert_eq!(count, 100, "all messages should be processed sequentially");
    }
}
