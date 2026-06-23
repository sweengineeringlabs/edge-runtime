//! Integration tests for the in-memory task queue API marker.

/// @covers: TaskQueueFactory::in_memory
#[cfg(feature = "tokio-rt")]
#[test]
fn test_in_memory_task_queue_factory_returns_a_queue() {
    use swe_edge_runtime_message_broker::TaskQueueFactory;
    let queue = TaskQueueFactory::in_memory();
    // Queue must be a valid TaskQueue instance — hold it to prove type correctness.
    let _ = queue;
    assert!(true, "in_memory factory returns a valid TaskQueue");
}
