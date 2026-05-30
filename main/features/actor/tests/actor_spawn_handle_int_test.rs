//! Integration tests for [`ActorSpawnHandle`].

#[cfg(feature = "tokio-rt")]
mod tokio_tests {
    use futures::future::BoxFuture;
    use swe_edge_runtime_actor::{Actor, ActorContext, ActorHandle, ActorRuntime};

    struct Counter {
        count: u32,
    }

    enum CounterMsg {
        Inc,
        Get(tokio::sync::oneshot::Sender<u32>),
    }

    impl Actor for Counter {
        type Message = CounterMsg;

        fn handle(&mut self, _ctx: ActorContext<Self>, msg: Self::Message) -> BoxFuture<'_, ()> {
            Box::pin(async move {
                match msg {
                    CounterMsg::Inc => self.count += 1,
                    CounterMsg::Get(tx) => {
                        let _ = tx.send(self.count);
                    }
                }
            })
        }
    }

    /// @covers: ActorSpawnHandle::clone
    #[tokio::test]
    async fn test_actor_spawn_handle_clone_shares_mailbox() {
        let handle = ActorRuntime::spawn(Counter { count: 0 });
        let handle2 = handle.clone();
        handle
            .tell(CounterMsg::Inc)
            .await
            .expect("tell must succeed");
        let (tx, rx) = tokio::sync::oneshot::channel();
        handle2
            .tell(CounterMsg::Get(tx))
            .await
            .expect("get must succeed");
        assert_eq!(rx.await.expect("reply"), 1);
    }

    /// @covers: ActorSpawnHandle::tell
    #[tokio::test]
    async fn test_actor_spawn_handle_tell_delivers_message() {
        let handle = ActorRuntime::spawn(Counter { count: 0 });
        handle
            .tell(CounterMsg::Inc)
            .await
            .expect("tell must succeed");
        handle
            .tell(CounterMsg::Inc)
            .await
            .expect("tell must succeed");
        let (tx, rx) = tokio::sync::oneshot::channel();
        handle
            .tell(CounterMsg::Get(tx))
            .await
            .expect("get must succeed");
        assert_eq!(rx.await.expect("reply"), 2);
    }
}
