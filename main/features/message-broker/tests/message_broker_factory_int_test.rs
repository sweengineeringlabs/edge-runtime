//! Integration tests for [`MessageBrokerFactory`].

use swe_edge_runtime_message_broker::MessageBrokerFactory;

/// @covers: MessageBrokerFactory::create_config_builder
#[test]
fn test_message_broker_factory_create_config_builder_is_pre_seeded() {
    use swe_edge_configbuilder::ConfigBuilder as _;
    let _loader = MessageBrokerFactory::create_config_builder().build_loader();
}

/// @covers: MessageBrokerFactory::in_memory
#[cfg(feature = "tokio-rt")]
#[test]
fn test_message_broker_factory_in_memory_returns_broker() {
    let _broker = MessageBrokerFactory::in_memory();
}
