//! Integration tests for the `StopHandle` trait.

#[cfg(feature = "tokio-rt")]
mod tests {
    use futures::future::BoxFuture;
    use swe_edge_runtime_actor::{
        Actor, ActorContext, ActorHandle, ActorRuntime, MailboxError, StopHandle,
    };

    struct Noop;

    impl Actor for Noop {
        type Message = ();

        fn handle(&mut self, _ctx: ActorContext<Self>, _msg: Self::Message) -> BoxFuture<'_, ()> {
            Box::pin(async move {})
        }
    }

    /// @covers: StopHandle::stop — actor rejects messages after stop
    #[tokio::test]
    async fn test_stop_handle_stop_causes_actor_to_reject_messages() {
        let (handle, stop) = ActorRuntime::spawn_with_stop(Noop);

        // Send one message before stopping
        assert!(handle.tell(()).await.is_ok());
        stop.stop().await;

        // Give actor loop time to process Stop signal
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;

        let result = handle.tell(()).await;
        assert!(
            matches!(result, Err(MailboxError::Closed)),
            "actor must not accept messages after stop"
        );
    }

    /// @covers: StopHandle::clone — cloned stop handle reaches same actor
    #[tokio::test]
    async fn test_stop_handle_clone_stops_same_actor() {
        let (handle, stop1) = ActorRuntime::spawn_with_stop(Noop);
        let stop2 = stop1.clone();

        stop2.stop().await; // stop via clone

        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;

        let result = handle.tell(()).await;
        assert!(
            result.is_err(),
            "actor stopped via cloned stop handle must reject messages"
        );
    }
}
