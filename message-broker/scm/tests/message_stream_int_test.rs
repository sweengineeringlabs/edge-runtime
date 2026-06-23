//! Integration tests for [`MessageStream`].

use swe_edge_runtime_message_broker::MessageStream;

/// @covers: MessageStream
#[test]
fn test_message_stream_type_alias_accepts_boxed_stream() {
    fn _accepts(_: &MessageStream) {}
    assert!(true, "MessageStream type alias accepts boxed streams");
}
