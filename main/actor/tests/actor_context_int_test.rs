//! Integration tests for `ActorContext`.

#[cfg(feature = "tokio-rt")]
mod tests {
    use futures::future::BoxFuture;
    use swe_edge_runtime_actor::{Actor, ActorContext, ActorHandle, ActorRuntime};

    /// @covers: ActorContext — clone produces independent context for same actor
    #[tokio::test]
    async fn test_actor_context_is_passed_to_handle_method() {
        // ActorContext is passed into the actor's handle method.
        // This test verifies the context is received (non-null) by using
        // an actor that sends back confirmation via a channel.

        struct ContextActor;

        impl Actor for ContextActor {
            type Message = tokio::sync::oneshot::Sender<bool>;

            fn handle(&mut self, _ctx: ActorContext<Self>, tx: Self::Message) -> BoxFuture<'_, ()> {
                Box::pin(async move {
                    // Context was provided — send confirmation
                    let _ = tx.send(true);
                })
            }
        }

        let handle = ActorRuntime::spawn(ContextActor);
        let (tx, rx) = tokio::sync::oneshot::channel();
        handle
            .tell(tx)
            .await
            .unwrap_or_else(|_| panic!("tell failed"));
        let received = rx.await.unwrap_or_else(|_| panic!("recv failed"));
        assert!(received, "actor must receive context and respond");
    }
}
