//! [`BrokerError`] — error variants for message broker operations.

/// Errors returned by [`crate::MessageBroker`] operations.
#[derive(Debug, thiserror::Error)]
pub enum BrokerError {
    /// A publish operation failed.
    #[error("publish to '{topic}' failed: {reason}")]
    Publish {
        /// The topic the publish was attempted on.
        topic: String,
        /// Human-readable failure description.
        reason: String,
    },

    /// A subscribe operation failed.
    #[error("subscribe to '{topic}' failed: {reason}")]
    Subscribe {
        /// The topic the subscribe was attempted on.
        topic: String,
        /// Human-readable failure description.
        reason: String,
    },

    /// The subscriber fell behind and messages were dropped by the broker.
    #[error("stream lagged: {count} messages dropped")]
    StreamLagged {
        /// Number of messages that were dropped.
        count: u64,
    },

    /// The broker connection could not be established or was lost.
    #[error("broker connection failed: {0}")]
    Connection(String),

    /// The broker is temporarily unavailable.
    #[error("broker unavailable: {0}")]
    Unavailable(String),
}
