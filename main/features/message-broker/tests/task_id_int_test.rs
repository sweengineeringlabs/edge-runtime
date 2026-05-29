//! Integration tests for [`TaskId`].

use swe_edge_runtime_message_broker::TaskId;

/// @covers: TaskId::new
#[test]
fn test_task_id_new_generates_unique_ids() {
    let id1 = TaskId::new();
    let id2 = TaskId::new();
    assert_ne!(id1, id2, "two new TaskIds must be distinct");
}
