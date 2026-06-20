//! Integration tests for the in-memory message broker API marker.

/// @covers: MessageBrokerFactory::in_memory
#[cfg(feature = "tokio-rt")]
#[test]
fn test_in_memory_broker_factory_returns_a_broker() {
    use swe_edge_runtime_message_broker::MessageBrokerFactory;
    let _ = MessageBrokerFactory::in_memory();
}
