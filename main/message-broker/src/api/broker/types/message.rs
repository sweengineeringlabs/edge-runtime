//! [`Message`] — the unit of exchange for [`crate::MessageBroker`].

use std::collections::HashMap;

use bytes::Bytes;

/// A message payload with optional metadata headers.
///
/// `Message` is the currency passed between producers and consumers.  It
/// carries raw bytes and an optional key-value header map for routing hints,
/// content-type annotations, or correlation IDs.
///
/// # Examples
///
/// ```rust
/// use swe_edge_runtime_message_broker::Message;
///
/// let msg = Message::new(b"event.payload".as_ref());
/// assert_eq!(msg.payload.as_ref(), b"event.payload");
/// assert!(msg.headers.is_empty());
///
/// let msg = Message::with_headers(
///     b"data".as_ref(),
///     [("x-source".to_string(), "order-service".to_string())].into(),
/// );
/// assert_eq!(msg.headers.get("x-source").map(String::as_str), Some("order-service"));
/// ```
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
