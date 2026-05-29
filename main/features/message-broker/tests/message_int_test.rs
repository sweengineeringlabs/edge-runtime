//! Integration tests for [`Message`].

use swe_edge_runtime_message_broker::Message;

/// @covers: Message::new
#[test]
fn test_message_new_creates_empty_headers() {
    let m = Message::new(b"hello".as_ref());
    assert_eq!(m.payload.as_ref(), b"hello");
    assert!(m.headers.is_empty(), "expected no headers");
}

/// @covers: Message::with_headers
#[test]
fn test_message_with_headers_stores_headers() {
    use std::collections::HashMap;
    let mut h = HashMap::new();
    h.insert("content-type".into(), "application/json".into());
    let m = Message::with_headers(b"{}".as_ref(), h);
    assert_eq!(
        m.headers.get("content-type").map(String::as_str),
        Some("application/json")
    );
}
