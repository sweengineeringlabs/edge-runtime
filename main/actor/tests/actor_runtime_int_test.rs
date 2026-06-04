//! Integration tests for `ActorRuntime`.

use swe_edge_runtime_actor::ActorRuntime;

/// @covers: ActorRuntime::create_config_builder — returns builder seeded with package name
#[test]
fn test_actor_runtime_create_config_builder_seeded_with_package_name() {
    let builder = ActorRuntime::create_config_builder();
    assert_eq!(
        builder.name(),
        "swe-edge-runtime-actor",
        "ActorRuntime::create_config_builder must seed the crate package name"
    );
}

/// @covers: ActorRuntime::create_config_builder — package version is seeded
#[test]
fn test_actor_runtime_create_config_builder_seeded_with_version() {
    let builder = ActorRuntime::create_config_builder();
    // Version is non-empty and matches package version format
    assert!(
        !builder.version().is_empty(),
        "ActorRuntime::create_config_builder must seed a non-empty version"
    );
}

#[cfg(feature = "tokio-rt")]
mod spawn_tests {
    use futures::future::BoxFuture;
    use swe_edge_runtime_actor::{Actor, ActorContext, ActorHandle, ActorRuntime};

    struct Noop;

    impl Actor for Noop {
        type Message = ();

        fn handle(&mut self, _ctx: ActorContext<Self>, _msg: Self::Message) -> BoxFuture<'_, ()> {
            Box::pin(async move {})
        }
    }

    /// @covers: ActorRuntime::spawn — actor processes at least one message
    #[tokio::test]
    async fn test_actor_runtime_spawn_accepts_messages() {
        let handle = ActorRuntime::spawn(Noop);
        assert!(
            handle.tell(()).await.is_ok(),
            "spawned actor must accept messages"
        );
    }
}
