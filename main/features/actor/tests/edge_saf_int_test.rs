//! Integration tests for SAF (Service Abstraction Framework) public factories.
//!
//! These tests verify that SAF factories:
//! - Hide implementation details (return `impl Trait`, never expose concrete types)
//! - Properly initialize actors
//! - Work end-to-end without exposing Cargo dependencies

#[cfg(feature = "tokio-rt")]
mod tokio_rt_tests {
    use futures::future::BoxFuture;
    use swe_edge_runtime_actor::{
        spawn_actor, spawn_actor_with_stop, Actor, ActorContext, ActorHandle, StopHandle,
    };

    struct SimpleActor;

    impl Actor for SimpleActor {
        type Message = ();

        fn handle(&mut self, _ctx: ActorContext<Self>, _msg: Self::Message) -> BoxFuture<'_, ()> {
            Box::pin(async move {})
        }
    }

    /// @covers: spawn_actor
    #[tokio::test]
    async fn test_spawn_actor_factory_returns_working_actor() {
        let actor = SimpleActor;
        let handle = spawn_actor(actor);

        assert!(handle.tell(()).await.is_ok());
    }

    /// @covers: spawn_actor_with_stop
    #[tokio::test]
    async fn test_spawn_actor_with_stop_factory_returns_both_handles() {
        let actor = SimpleActor;
        let (handle, stop) = spawn_actor_with_stop(actor);

        assert!(handle.tell(()).await.is_ok());
        stop.stop().await;
    }

    /// @covers: spawn_actor
    #[tokio::test]
    async fn test_spawn_actor_factory_hides_implementation_types() {
        let actor = SimpleActor;
        let handle = spawn_actor(actor);

        // The handle is `impl ActorHandle`, not a concrete type.
        // This test documents that concrete types (TokioActorHandle) are never exposed.
        let _ = handle.clone();
    }
}
