//! [`Message`] — the unit of exchange for [`crate::MessageBroker`].

use std::collections::HashMap;

use bytes::Bytes;

/// A message payload with optional metadata headers.
///
/// `Message` is the currency passed between producers and consumers.  It
/// carries raw bytes and an optional key-value header map for routing hints,
/// content-type annotations, or correlation IDs.
#[derive(Debug, Clone)]
pub struct Message {
    /// Raw bytes payload.
    pub payload: Bytes,
    /// Optional key-value metadata headers.
    pub headers: HashMap<String, String>,
}

impl Message {
    /// Construct a message from raw bytes with no headers.
    pub fn new(payload: impl Into<Bytes>) -> Self {
        Self {
            payload: payload.into(),
            headers: HashMap::new(),
        }
    }

    /// Construct a message with headers.
    pub fn with_headers(payload: impl Into<Bytes>, headers: HashMap<String, String>) -> Self {
        Self {
            payload: payload.into(),
            headers,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_creates_message_with_empty_headers() {
        let m = Message::new(b"hello".as_ref());
        assert_eq!(m.payload.as_ref(), b"hello");
        assert!(m.headers.is_empty());
    }

    /// @covers: with_headers
    #[test]
    fn test_with_headers_stores_provided_headers() {
        let mut h = HashMap::new();
        h.insert("content-type".into(), "application/json".into());
        let m = Message::with_headers(b"{}".as_ref(), h);
        assert_eq!(
            m.headers.get("content-type").map(String::as_str),
            Some("application/json")
        );
    }

    #[test]
    fn test_message_clone_produces_independent_copy() {
        let m = Message::new(b"data".as_ref());
        let m2 = m.clone();
        assert_eq!(m.payload, m2.payload);
    }
}
