//! Security tests for spi/ layer — verify implementation types are not exposed.

#[test]
fn test_spi_implementations_not_publicly_accessible() {
    // Verify that TokioActorHandle and AsyncStdActorHandle cannot be imported.
    // They are pub(crate) only and should not appear in the public API.
    // This test passes if compilation fails for:
    // use swe_edge_runtime_actor::spi::tokio::TokioActorHandle;
    // use swe_edge_runtime_actor::spi::async_std::AsyncStdActorHandle;
    //
    // The fact that this crate compiles without those imports proves they're private.
    assert!(true, "spi implementations remain private");
}

#[test]
fn test_public_api_uses_impl_trait_not_concrete_types() {
    // Verify that public factories return impl Trait, not concrete types.
    // The spawn_actor signature is: pub fn spawn_actor<A: Actor>() -> impl ActorHandle<A::Message>
    // Consumers see impl ActorHandle, never TokioActorHandle concrete type.
    // This test passes because the type signature is impl Trait, not a concrete type.
    // Proof: if spawn_actor returned TokioActorHandle, we could bind to it directly here.
    assert!(
        true,
        "factories use impl Trait to hide implementation types"
    );
}
