//! Integration tests for [`BrokerError`].

use swe_edge_runtime_message_broker::BrokerError;

/// @covers: BrokerError::Publish
#[test]
fn test_broker_error_publish_includes_topic_and_reason() {
    let e = BrokerError::Publish {
        topic: "orders".into(),
        reason: "no receivers".into(),
    };
    let msg = e.to_string();
    assert!(
        msg.contains("orders"),
        "expected topic in error message: {msg}"
    );
    assert!(
        msg.contains("no receivers"),
        "expected reason in error message: {msg}"
    );
}

/// @covers: BrokerError::StreamLagged
#[test]
fn test_broker_error_stream_lagged_includes_count() {
    let e = BrokerError::StreamLagged { count: 42 };
    assert!(
        e.to_string().contains("42"),
        "expected count in error message"
    );
}

/// @covers: BrokerError::Connection
#[test]
fn test_broker_error_connection_displays_reason() {
    let e = BrokerError::Connection("refused".into());
    assert!(
        e.to_string().contains("refused"),
        "expected reason in error message"
    );
}
