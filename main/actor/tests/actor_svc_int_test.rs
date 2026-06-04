//! Integration tests for the actor_svc SAF layer (ActorRuntime factory methods).

use swe_edge_runtime_actor::ActorRuntime;

/// @covers: ActorRuntime::create_config_builder
#[test]
fn test_create_config_builder_is_pre_seeded() {
    let _loader = ActorRuntime::create_config_builder().build_loader();
}

/// @covers: ActorRuntime::create_config_builder — package name is seeded
#[test]
fn test_create_config_builder_has_package_name() {
    let builder = ActorRuntime::create_config_builder();
    assert_eq!(
        builder.name(),
        "swe-edge-runtime-actor",
        "create_config_builder must pre-seed the package name"
    );
}

#[cfg(feature = "tokio-rt")]
mod spawn_tests {
    use futures::future::BoxFuture;
    use swe_edge_runtime_actor::{Actor, ActorContext, ActorHandle, ActorRuntime, StopHandle};

    struct Ping;

    impl Actor for Ping {
        type Message = tokio::sync::oneshot::Sender<bool>;

        fn handle(&mut self, _ctx: ActorContext<Self>, tx: Self::Message) -> BoxFuture<'_, ()> {
            Box::pin(async move {
                let _ = tx.send(true);
            })
        }
    }

    /// @covers: ActorRuntime::spawn
    #[tokio::test]
    async fn test_actor_runtime_spawn_returns_functioning_handle() {
        let handle = ActorRuntime::spawn(Ping);
        let (tx, rx) = tokio::sync::oneshot::channel();
        handle
            .tell(tx)
            .await
            .unwrap_or_else(|_| panic!("tell failed"));
        let pong = rx.await.unwrap_or_else(|_| panic!("recv failed"));
        assert!(pong, "actor must respond true to Ping");
    }

    /// @covers: ActorRuntime::spawn_with_stop
    #[tokio::test]
    async fn test_actor_runtime_spawn_with_stop_returns_both_handles() {
        let (handle, stop) = ActorRuntime::spawn_with_stop(Ping);
        let (tx, rx) = tokio::sync::oneshot::channel();
        handle
            .tell(tx)
            .await
            .unwrap_or_else(|_| panic!("tell failed"));
        let pong = rx.await.unwrap_or_else(|_| panic!("recv failed"));
        assert!(pong, "actor must respond before stop");
        stop.stop().await;
    }
}
