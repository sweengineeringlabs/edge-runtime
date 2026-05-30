//! Broker core layer — shared constants for broker backends.

/// Maximum topic name length in bytes enforced on publish and subscribe calls.
pub(crate) const MAX_TOPIC_BYTES: usize = 256;

/// Maximum number of headers per message.
pub(crate) const MAX_MESSAGE_HEADERS: usize = 64;

/// Maximum header key length in bytes.
pub(crate) const MAX_HEADER_KEY_BYTES: usize = 128;

/// Maximum header value length in bytes.
pub(crate) const MAX_HEADER_VALUE_BYTES: usize = 1024;
