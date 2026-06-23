//! Integration tests for [`MessageBrokerFactory`].

use swe_edge_runtime_message_broker::MessageBrokerFactory;

/// @covers: MessageBrokerFactory::create_config_builder
#[test]
fn test_message_broker_factory_create_config_builder_is_pre_seeded() {
    let builder = MessageBrokerFactory::create_config_builder();
    let loader = builder.build_loader();
    assert!(loader.is_ok(), "builder must construct a valid loader");
    let _ = loader.unwrap();
}

/// @covers: MessageBrokerFactory::in_memory
#[cfg(feature = "tokio-rt")]
#[test]
fn test_message_broker_factory_in_memory_returns_broker() {
    use swe_edge_message_broker::MessageBroker;
    let broker = MessageBrokerFactory::in_memory();
    let rt = tokio::runtime::Builder::new_current_thread()
        .build()
        .expect("tokio rt");
    let health = rt.block_on(broker.health_check());
    assert_eq!(health, Ok(()), "in-memory broker must be healthy");
}
