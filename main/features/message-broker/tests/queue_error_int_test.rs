//! Integration tests for [`QueueError`].

use swe_edge_runtime_message_broker::QueueError;

/// @covers: QueueError::Enqueue
#[test]
fn test_queue_error_enqueue_includes_reason() {
    let e = QueueError::Enqueue("serialization failed".into());
    assert!(e.to_string().contains("serialization failed"));
}

/// @covers: QueueError::Dequeue
#[test]
fn test_queue_error_dequeue_includes_reason() {
    let e = QueueError::Dequeue("connection lost".into());
    assert!(e.to_string().contains("connection lost"));
}

/// @covers: QueueError::Full
#[test]
fn test_queue_error_full_displays() {
    let e = QueueError::Full;
    assert_eq!(e.to_string(), "queue full");
}

/// @covers: QueueError::Closed
#[test]
fn test_queue_error_closed_displays() {
    let e = QueueError::Closed;
    assert_eq!(e.to_string(), "queue closed");
}
