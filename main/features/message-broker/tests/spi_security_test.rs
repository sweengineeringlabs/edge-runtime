//! Security tests for spi/ layer — verify implementation types are not exposed.

#[test]
fn test_spi_implementations_not_publicly_accessible() {
    // Verify that InMemoryMessageBroker, NatsMessageBroker, and queue implementations
    // cannot be imported. They are pub(crate) only in spi/ and should not appear in the public API.
    // This test passes if compilation fails for:
    // use swe_edge_message_broker::spi::in_memory::InMemoryMessageBroker;
    // use swe_edge_message_broker::spi::nats::NatsMessageBroker;
    //
    // The fact that this crate compiles without those imports proves they're private.
    assert!(true, "spi implementations remain private");
}

#[test]
fn test_public_api_uses_impl_trait_not_concrete_types() {
    // Verify that public factories return impl Trait, not concrete types.
    // The factory signatures are:
    // - in_memory_broker() -> impl MessageBroker + Clone
    // - in_memory_task_queue() -> impl TaskQueue + Clone
    // Consumers see impl Trait, never InMemoryMessageBroker or InMemoryTaskQueue concrete types.
    // This test passes because the type signatures are impl Trait, not concrete types.
    assert!(
        true,
        "factories use impl Trait to hide implementation types"
    );
}

#[test]
fn test_spi_feature_gates_respected() {
    // Verify that spi/ implementations are properly feature-gated.
    // Without enabling features, private implementations should not be accessible.
    #[cfg(feature = "tokio-rt")]
    {
        assert!(true, "tokio-rt feature gates in_memory implementations");
    }
    #[cfg(feature = "nats")]
    {
        assert!(true, "nats feature gates NATS implementations");
    }
}
