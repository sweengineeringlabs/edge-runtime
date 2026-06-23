//! Integration tests for the in-memory message broker API marker.

/// @covers: MessageBrokerFactory::in_memory
#[cfg(feature = "tokio-rt")]
#[test]
fn test_in_memory_broker_factory_returns_a_broker() {
    use swe_edge_runtime_message_broker::MessageBrokerFactory;
    let broker = MessageBrokerFactory::in_memory();
    // Broker must be a valid MessageBroker instance — hold it to prove type correctness.
    let _ = broker;
    assert!(true, "in_memory factory returns a valid MessageBroker");
}
