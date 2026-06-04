//! Integration tests for [`TaskQueueFactory`].

use swe_edge_runtime_message_broker::TaskQueueFactory;

/// @covers: TaskQueueFactory::in_memory
#[cfg(feature = "tokio-rt")]
#[test]
fn test_task_queue_factory_in_memory_returns_queue() {
    let _queue = TaskQueueFactory::in_memory();
}
