//! Integration tests for the async-std actor backend.
//!
//! Exercises the async-std dependency to satisfy rule 95 (deps used in src/ must
//! have integration or e2e test coverage).

#[cfg(feature = "async-std-rt")]
mod async_std_rt_tests {
    use futures::future::BoxFuture;
    use swe_edge_runtime_actor::{Actor, ActorContext, ActorHandle, ActorRuntime, StopHandle};

    struct Counter {
        count: i32,
    }

    enum CounterMessage {
        Increment,
        GetCount(async_std::channel::Sender<i32>),
    }

    impl Actor for Counter {
        type Message = CounterMessage;

        fn handle(&mut self, _ctx: ActorContext<Self>, msg: Self::Message) -> BoxFuture<'_, ()> {
            Box::pin(async move {
                match msg {
                    CounterMessage::Increment => self.count += 1,
                    CounterMessage::GetCount(tx) => {
                        let _ = tx.send(self.count).await;
                    }
                }
            })
        }
    }

    /// @covers: async-std actor spawn sequential message processing
    #[async_std::test]
    async fn test_async_std_actor_processes_messages_sequentially() {
        let actor = Counter { count: 0 };
        let handle = ActorRuntime::spawn(actor);

        handle
            .tell(CounterMessage::Increment)
            .await
            .unwrap_or_else(|_| panic!("tell failed"));

        let (tx, rx) = async_std::channel::bounded(1);
        handle
            .tell(CounterMessage::GetCount(tx))
            .await
            .unwrap_or_else(|_| panic!("tell failed"));
        let count = rx.recv().await.unwrap_or_else(|_| panic!("recv failed"));
        assert_eq!(count, 1, "message must be processed before GetCount");
    }
}

// Verify async-std channel API is accessible — confirms dev-dep resolution.
// Full async-std actor tests run with --features async-std-rt.
#[test]
fn test_async_std_channel_bounded_creates_channel() {
    let (tx, _rx) = async_std::channel::bounded::<u32>(1);
    // Channel creation succeeds — confirming async-std is linked and accessible.
    drop(tx);
}
