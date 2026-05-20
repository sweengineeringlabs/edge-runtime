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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_publish_error_includes_topic_and_reason() {
        let e = BrokerError::Publish {
            topic: "orders".into(),
            reason: "no receivers".into(),
        };
        let msg = e.to_string();
        assert!(msg.contains("orders"), "expected topic in message: {msg}");
        assert!(
            msg.contains("no receivers"),
            "expected reason in message: {msg}"
        );
    }

    #[test]
    fn test_stream_lagged_includes_count() {
        let e = BrokerError::StreamLagged { count: 42 };
        assert!(e.to_string().contains("42"));
    }

    #[test]
    fn test_connection_error_displays_reason() {
        let e = BrokerError::Connection("refused".into());
        assert!(e.to_string().contains("refused"));
    }
}
