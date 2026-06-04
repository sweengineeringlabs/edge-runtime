//! Integration tests for Actor trait implementations.

#[cfg(feature = "tokio-rt")]
mod tokio_rt_tests {
    use futures::future::BoxFuture;
    use swe_edge_runtime_actor::{
        Actor, ActorContext, ActorHandle, ActorRuntime, MailboxError, StopHandle,
    };

    #[derive(Clone)]
    struct Counter {
        count: i32,
    }

    enum CounterMessage {
        Increment,
        Decrement,
        GetCount(tokio::sync::oneshot::Sender<i32>),
    }

    impl Actor for Counter {
        type Message = CounterMessage;

        fn handle(&mut self, _ctx: ActorContext<Self>, msg: Self::Message) -> BoxFuture<'_, ()> {
            Box::pin(async move {
                match msg {
                    CounterMessage::Increment => self.count += 1,
                    CounterMessage::Decrement => self.count -= 1,
                    CounterMessage::GetCount(tx) => {
                        let _ = tx.send(self.count);
                    }
                }
            })
        }

        fn on_stop(&mut self) -> BoxFuture<'_, ()> {
            Box::pin(async move {
                // Cleanup: in real use case, could flush state, close files, etc.
            })
        }
    }

    /// @covers: ActorRuntime::spawn
    #[tokio::test]
    async fn test_spawn_actor_returns_working_handle() {
        let counter = Counter { count: 0 };
        let handle = ActorRuntime::spawn(counter);

        assert!(handle.tell(CounterMessage::Increment).await.is_ok());

        let (tx, rx) = tokio::sync::oneshot::channel();
        handle
            .tell(CounterMessage::GetCount(tx))
            .await
            .unwrap_or_else(|_| panic!("tell failed"));
        let count = rx.await.unwrap_or_else(|_| panic!("recv failed"));
        assert_eq!(count, 1);
    }

    /// @covers: Actor::handle
    #[tokio::test]
    async fn test_actor_sequential_message_processing() {
        let counter = Counter { count: 10 };
        let handle = ActorRuntime::spawn(counter);

        handle
            .tell(CounterMessage::Increment)
            .await
            .unwrap_or_else(|_| panic!("tell failed"));
        handle
            .tell(CounterMessage::Increment)
            .await
            .unwrap_or_else(|_| panic!("tell failed"));
        handle
            .tell(CounterMessage::Decrement)
            .await
            .unwrap_or_else(|_| panic!("tell failed"));

        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;

        let (tx, rx) = tokio::sync::oneshot::channel();
        handle
            .tell(CounterMessage::GetCount(tx))
            .await
            .unwrap_or_else(|_| panic!("tell failed"));
        let count = rx.await.unwrap_or_else(|_| panic!("recv failed"));
        assert_eq!(count, 11, "messages should be processed sequentially");
    }

    /// @covers: ActorHandle::tell
    #[tokio::test]
    async fn test_tell_fire_and_forget() {
        let counter = Counter { count: 0 };
        let handle = ActorRuntime::spawn(counter);

        handle
            .tell(CounterMessage::Increment)
            .await
            .unwrap_or_else(|_| panic!("tell failed"));
        handle
            .tell(CounterMessage::Increment)
            .await
            .unwrap_or_else(|_| panic!("tell failed"));

        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;

        let (tx, rx) = tokio::sync::oneshot::channel();
        handle
            .tell(CounterMessage::GetCount(tx))
            .await
            .unwrap_or_else(|_| panic!("tell failed"));
        let count = rx.await.unwrap_or_else(|_| panic!("recv failed"));
        assert_eq!(count, 2);
    }

    /// @covers: ActorHandle::clone
    #[tokio::test]
    async fn test_cloning_handle_creates_additional_sender() {
        let counter = Counter { count: 0 };
        let handle1 = ActorRuntime::spawn(counter);
        let handle2 = handle1.clone();

        handle1
            .tell(CounterMessage::Increment)
            .await
            .unwrap_or_else(|_| panic!("tell failed"));
        handle2
            .tell(CounterMessage::Increment)
            .await
            .unwrap_or_else(|_| panic!("tell failed"));

        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;

        let (tx, rx) = tokio::sync::oneshot::channel();
        handle1
            .tell(CounterMessage::GetCount(tx))
            .await
            .unwrap_or_else(|_| panic!("tell failed"));
        let count = rx.await.unwrap_or_else(|_| panic!("recv failed"));
        assert_eq!(count, 2);
    }

    /// @covers: ActorRuntime::spawn_with_stop
    #[tokio::test]
    async fn test_spawn_actor_with_stop_allows_graceful_shutdown() {
        let counter = Counter { count: 0 };
        let (handle, stop) = ActorRuntime::spawn_with_stop(counter);

        handle
            .tell(CounterMessage::Increment)
            .await
            .unwrap_or_else(|_| panic!("tell failed"));
        stop.stop().await;

        // Give actor loop time to process Stop signal
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;

        let result = handle.tell(CounterMessage::Increment).await;
        assert!(
            matches!(result, Err(MailboxError::Closed)),
            "should reject messages after stop"
        );
    }

    /// @covers: Actor::on_stop
    #[tokio::test]
    async fn test_on_stop_called_on_graceful_shutdown() {
        let counter = Counter { count: 0 };
        let (handle, stop) = ActorRuntime::spawn_with_stop(counter);

        handle
            .tell(CounterMessage::Increment)
            .await
            .unwrap_or_else(|_| panic!("tell failed"));
        stop.stop().await;

        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
    }

    /// @covers: MailboxError::ReplyDropped
    #[tokio::test]
    async fn test_actor_reply_channel_drop() {
        struct DoNothing;

        impl Actor for DoNothing {
            type Message = tokio::sync::oneshot::Sender<i32>;

            fn handle(
                &mut self,
                _ctx: ActorContext<Self>,
                _msg: Self::Message,
            ) -> BoxFuture<'_, ()> {
                Box::pin(async move {
                    // Intentionally don't respond — reply channel will drop
                })
            }
        }

        let actor = DoNothing;
        let handle = ActorRuntime::spawn(actor);

        let (tx, rx) = tokio::sync::oneshot::channel();
        handle
            .tell(tx)
            .await
            .unwrap_or_else(|_| panic!("tell failed"));

        // Wait for processing, then rx will be dropped by actor
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
        let result = rx.await;
        assert!(
            result.is_err(),
            "reply channel should be dropped when not responded"
        );
    }
}
