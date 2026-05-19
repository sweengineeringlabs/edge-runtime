//! SAF — actor spawn factories.

use crate::api::{Actor, ActorHandle, StopHandle};
use crate::core::actor::{spawn_tokio_actor, spawn_tokio_actor_with_stop};

/// Spawn an actor with no lifecycle management.
///
/// The actor runs in a spawned tokio task, processing messages sequentially.
/// Returns a handle to send messages; the actor stops when all handles are dropped.
///
/// # Example
/// ```ignore
/// let handle = spawn_actor(Counter { count: 0 });
/// handle.tell(CounterMessage::Increment).await?;
/// ```
///
/// Requires the `tokio-rt` feature.
#[cfg(feature = "tokio-rt")]
pub fn spawn_actor<A: Actor>(actor: A) -> impl ActorHandle<A::Message> {
    spawn_tokio_actor(actor)
}

/// Spawn an actor with explicit lifecycle management.
///
/// Returns both a message handle and a stop handle. Calling `stop()` signals the
/// actor to shut down gracefully after processing the current message.
///
/// # Example
/// ```ignore
/// let (handle, stop) = spawn_actor_with_stop(Counter { count: 0 });
/// handle.tell(CounterMessage::Increment).await?;
/// stop.stop().await;
/// ```
///
/// Requires the `tokio-rt` feature.
#[cfg(feature = "tokio-rt")]
pub fn spawn_actor_with_stop<A: Actor>(
    actor: A,
) -> (impl ActorHandle<A::Message>, impl StopHandle) {
    spawn_tokio_actor_with_stop(actor)
}

#[cfg(test)]
mod tests {
    use futures::future::BoxFuture;

    use crate::api::{ActorHandle, StopHandle};

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

        fn handle(
            &mut self,
            _ctx: crate::api::ActorContext<Self>,
            msg: Self::Message,
        ) -> BoxFuture<'_, ()> {
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

    /// @covers: spawn_actor
    #[cfg(feature = "tokio-rt")]
    #[tokio::test]
    async fn test_spawn_actor_factory_returns_working_handle() {
        let actor = TestActor { count: 0 };
        let handle = spawn_actor(actor);

        handle.tell(TestMessage::Inc).await.unwrap();
        let (tx, rx) = tokio::sync::oneshot::channel();
        handle.tell(TestMessage::GetCount(tx)).await.unwrap();
        let count = rx.await.unwrap();
        assert_eq!(count, 1);
    }

    /// @covers: spawn_actor_with_stop
    #[cfg(feature = "tokio-rt")]
    #[tokio::test]
    async fn test_spawn_actor_with_stop_factory_returns_both_handles() {
        let actor = TestActor { count: 0 };
        let (handle, stop) = spawn_actor_with_stop(actor);

        handle.tell(TestMessage::Inc).await.unwrap();
        stop.stop().await;

        // Give actor loop time to process Stop signal
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;

        let result = handle.tell(TestMessage::Inc).await;
        assert!(result.is_err());
    }
}
