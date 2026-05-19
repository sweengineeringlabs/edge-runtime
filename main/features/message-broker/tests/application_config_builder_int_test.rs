//! Integration tests for [`ApplicationConfigBuilder`].

use swe_edge_message_broker::ApplicationConfigBuilder;

/// @covers: ApplicationConfigBuilder
#[test]
fn test_application_config_builder_is_constructible() {
    let _ = ApplicationConfigBuilder;
}
