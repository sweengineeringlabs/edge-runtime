//! Integration tests for the [`MessageBroker`] trait contract.

use swe_edge_runtime_message_broker::MessageBroker;

/// @covers: MessageBroker
#[test]
fn test_message_broker_trait_is_object_safe() {
    fn _check(_: &dyn MessageBroker) {}
}
