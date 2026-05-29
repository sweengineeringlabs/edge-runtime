//! Integration tests for the `ActorHandle` trait.

#[cfg(feature = "tokio-rt")]
mod tests {
    use futures::future::BoxFuture;
    use swe_edge_runtime_actor::{Actor, ActorContext, ActorHandle, ActorRuntime, MailboxError};

    struct Pinger;

    impl Actor for Pinger {
        type Message = tokio::sync::oneshot::Sender<bool>;

        fn handle(&mut self, _ctx: ActorContext<Self>, tx: Self::Message) -> BoxFuture<'_, ()> {
            Box::pin(async move {
                let _ = tx.send(true);
            })
        }
    }

    /// @covers: ActorHandle::tell — message delivery confirmed
    #[tokio::test]
    async fn test_actor_handle_tell_delivers_message() {
        let handle = ActorRuntime::spawn(Pinger);
        let (tx, rx) = tokio::sync::oneshot::channel();
        handle
            .tell(tx)
            .await
            .unwrap_or_else(|_| panic!("tell failed"));
        let received = rx.await.unwrap_or_else(|_| panic!("recv failed"));
        assert!(received, "actor must respond true");
    }

    /// @covers: ActorHandle::clone — cloned handle reaches same actor
    #[tokio::test]
    async fn test_actor_handle_clone_shares_same_actor() {
        struct Counter {
            count: u32,
        }

        enum Msg {
            Inc,
            Get(tokio::sync::oneshot::Sender<u32>),
        }

        impl Actor for Counter {
            type Message = Msg;

            fn handle(
                &mut self,
                _ctx: ActorContext<Self>,
                msg: Self::Message,
            ) -> BoxFuture<'_, ()> {
                Box::pin(async move {
                    match msg {
                        Msg::Inc => self.count += 1,
                        Msg::Get(tx) => {
                            let _ = tx.send(self.count);
                        }
                    }
                })
            }
        }

        let h1 = ActorRuntime::spawn(Counter { count: 0 });
        let h2 = h1.clone();

        h1.tell(Msg::Inc)
            .await
            .unwrap_or_else(|_| panic!("tell h1 failed"));
        h2.tell(Msg::Inc)
            .await
            .unwrap_or_else(|_| panic!("tell h2 failed"));

        tokio::time::sleep(tokio::time::Duration::from_millis(5)).await;

        let (tx, rx) = tokio::sync::oneshot::channel();
        h1.tell(Msg::Get(tx))
            .await
            .unwrap_or_else(|_| panic!("tell get failed"));
        let count = rx.await.unwrap_or_else(|_| panic!("recv failed"));
        assert_eq!(count, 2, "both handles must reach the same actor");
    }

    /// @covers: ActorHandle::tell — returns Err(MailboxError::Closed) after actor stops
    #[tokio::test]
    async fn test_actor_handle_tell_returns_error_when_actor_stopped() {
        use swe_edge_runtime_actor::StopHandle;

        let (handle, stop) = ActorRuntime::spawn_with_stop(Pinger);
        stop.stop().await;

        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;

        let (tx, _rx) = tokio::sync::oneshot::channel();
        let result = handle.tell(tx).await;
        assert!(
            matches!(result, Err(MailboxError::Closed)),
            "should reject messages after stop"
        );
    }
}
