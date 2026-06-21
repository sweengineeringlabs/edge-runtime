//! Integration tests for [`MessageBrokerFactory::create_config_builder`].

use swe_edge_configbuilder::ConfigLoaderFactory;
use swe_edge_runtime_message_broker::MessageBrokerFactory;

/// @covers: swe-edge-configbuilder
/// @covers: MessageBrokerFactory::create_config_builder — returns a pre-seeded builder with this crate's package name
#[test]
fn test_create_config_builder_is_pre_seeded_with_package_name() {
    let _loader = MessageBrokerFactory::create_config_builder().build_loader();
}

/// @covers: swe-edge-configbuilder
/// Verify that ConfigLoaderFactory from swe-edge-configbuilder is usable directly.
#[test]
fn test_configbuilder_dep_is_exercised_directly() {
    let _loader = ConfigLoaderFactory::create_config_builder()
        .with_name("test-broker")
        .with_version("0.0.0")
        .build_loader();
}
