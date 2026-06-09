//! Integration tests for [`TaskHandle`].

use swe_edge_runtime_message_broker::{QueueError, Task, TaskHandle};

/// @covers: TaskHandle::new — task_id mirrors task.id for convenience.
#[test]
fn test_task_handle_new_task_id_mirrors_task_id() {
    let task = Task::new(b"payload".as_ref());
    let expected_id = task.id;
    let ack: futures::future::BoxFuture<'static, Result<(), QueueError>> =
        Box::pin(async { Ok(()) });
    let nack: futures::future::BoxFuture<'static, Result<(), QueueError>> =
        Box::pin(async { Ok(()) });
    let handle = TaskHandle::new(task, ack, nack);
    assert_eq!(handle.task_id, expected_id);
    assert_eq!(handle.task.id, expected_id);
}

/// @covers: TaskHandle::new — task payload is accessible after construction.
#[test]
fn test_task_handle_new_task_payload_is_accessible() {
    let payload = b"test-payload";
    let task = Task::new(payload.as_ref());
    let ack: futures::future::BoxFuture<'static, Result<(), QueueError>> =
        Box::pin(async { Ok(()) });
    let nack: futures::future::BoxFuture<'static, Result<(), QueueError>> =
        Box::pin(async { Ok(()) });
    let handle = TaskHandle::new(task, ack, nack);
    assert_eq!(
        handle.task.payload.as_ref(),
        payload,
        "task payload must be accessible via handle.task.payload"
    );
}
