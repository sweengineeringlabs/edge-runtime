//! Integration tests for [`TaskHandle`].

use swe_edge_runtime_message_broker::{QueueError, TaskHandle, TaskId};

/// @covers: TaskHandle::new
#[test]
fn test_task_handle_new_stores_task_id() {
    let task_id = TaskId::new();
    let ack: futures::future::BoxFuture<'static, Result<(), QueueError>> =
        Box::pin(async { Ok(()) });
    let nack: futures::future::BoxFuture<'static, Result<(), QueueError>> =
        Box::pin(async { Ok(()) });
    let handle = TaskHandle::new(task_id, ack, nack);
    assert_eq!(handle.task_id, task_id);
}
