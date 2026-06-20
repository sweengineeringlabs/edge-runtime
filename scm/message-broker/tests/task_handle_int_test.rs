//! Integration tests for [`TaskHandle`].

use std::collections::HashMap;

use bytes::Bytes;
use swe_edge_runtime_message_broker::{QueueError, Task, TaskHandle, TaskId};

/// @covers: TaskHandle::new — task_id is stored correctly.
#[test]
fn test_task_handle_new_task_id_is_stored() {
    let id = TaskId::new();
    let payload = Bytes::from_static(b"payload");
    let ack: futures::future::BoxFuture<'static, Result<(), QueueError>> =
        Box::pin(async { Ok(()) });
    let nack: futures::future::BoxFuture<'static, Result<(), QueueError>> =
        Box::pin(async { Ok(()) });
    let handle = TaskHandle::new(id, payload, HashMap::new(), ack, nack);
    assert_eq!(handle.task_id, id);
}

/// @covers: TaskHandle::new — payload is accessible after construction.
#[test]
fn test_task_handle_new_payload_is_accessible() {
    let payload_bytes = b"test-payload";
    let task = Task::new(payload_bytes.as_ref());
    let id = task.id;
    let ack: futures::future::BoxFuture<'static, Result<(), QueueError>> =
        Box::pin(async { Ok(()) });
    let nack: futures::future::BoxFuture<'static, Result<(), QueueError>> =
        Box::pin(async { Ok(()) });
    let handle = TaskHandle::new(id, task.payload, task.headers, ack, nack);
    assert_eq!(
        handle.payload.as_ref(),
        payload_bytes,
        "task payload must be accessible via handle.payload"
    );
}

/// @covers: TaskHandle::new — headers are stored and accessible.
#[test]
fn test_task_handle_new_headers_are_stored() {
    let id = TaskId::new();
    let payload = Bytes::from_static(b"data");
    let mut headers = HashMap::new();
    headers.insert("x-trace-id".to_owned(), "abc123".to_owned());
    let ack: futures::future::BoxFuture<'static, Result<(), QueueError>> =
        Box::pin(async { Ok(()) });
    let nack: futures::future::BoxFuture<'static, Result<(), QueueError>> =
        Box::pin(async { Ok(()) });
    let handle = TaskHandle::new(id, payload, headers.clone(), ack, nack);
    assert_eq!(
        handle.headers, headers,
        "headers must be preserved through TaskHandle"
    );
}
