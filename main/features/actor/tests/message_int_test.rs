//! Integration tests for the `Message` envelope type.
//!
//! `Message` is an internal spi/ type but is part of the api/types/ module.
//! These tests verify the message envelope correctly wraps user payloads.

#[cfg(feature = "tokio-rt")]
mod tests {
    use futures::future::BoxFuture;
    use swe_edge_runtime_actor::{Actor, ActorContext, ActorHandle, ActorRuntime};

    /// @covers: Message::Msg — payload is delivered to actor
    #[tokio::test]
    async fn test_message_msg_variant_delivers_payload_to_actor() {
        struct Doubler {
            last: i32,
        }

        enum Msg {
            Set(i32),
            Get(tokio::sync::oneshot::Sender<i32>),
        }

        impl Actor for Doubler {
            type Message = Msg;

            fn handle(
                &mut self,
                _ctx: ActorContext<Self>,
                msg: Self::Message,
            ) -> BoxFuture<'_, ()> {
                Box::pin(async move {
                    match msg {
                        Msg::Set(n) => self.last = n * 2,
                        Msg::Get(tx) => {
                            let _ = tx.send(self.last);
                        }
                    }
                })
            }
        }

        let handle = ActorRuntime::spawn(Doubler { last: 0 });
        handle
            .tell(Msg::Set(21))
            .await
            .unwrap_or_else(|_| panic!("tell failed"));

        let (tx, rx) = tokio::sync::oneshot::channel();
        handle
            .tell(Msg::Get(tx))
            .await
            .unwrap_or_else(|_| panic!("tell failed"));
        let result = rx.await.unwrap_or_else(|_| panic!("recv failed"));
        assert_eq!(result, 42, "Message::Msg must deliver payload; 21*2==42");
    }

    /// @covers: Message::Stop — actor stops after Stop message
    #[tokio::test]
    async fn test_message_stop_variant_terminates_actor() {
        use swe_edge_runtime_actor::StopHandle;

        struct Noop;

        impl Actor for Noop {
            type Message = ();

            fn handle(
                &mut self,
                _ctx: ActorContext<Self>,
                _msg: Self::Message,
            ) -> BoxFuture<'_, ()> {
                Box::pin(async move {})
            }
        }

        let (handle, stop) = ActorRuntime::spawn_with_stop(Noop);
        stop.stop().await;

        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;

        let result = handle.tell(()).await;
        assert!(result.is_err(), "Message::Stop must terminate the actor");
    }
}
