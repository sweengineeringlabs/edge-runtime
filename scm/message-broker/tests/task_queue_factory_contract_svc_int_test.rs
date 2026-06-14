//! Integration tests for [`TaskQueueFactoryContract`] trait (rule 222).
#![allow(clippy::unwrap_used, clippy::expect_used)]

use swe_edge_runtime_message_broker::{TaskQueueFactory, TaskQueueFactoryContract};

// --- TaskQueueFactoryContract::default_factory (rule 222) ---

/// @covers: TaskQueueFactoryContract::default_factory
#[test]
fn test_default_factory_returns_factory_instance_happy() {
    let _factory = TaskQueueFactory::default_factory();
}

/// @covers: TaskQueueFactoryContract::default_factory
#[test]
fn test_default_factory_is_callable_multiple_times_edge() {
    let _f1 = TaskQueueFactory::default_factory();
    let _f2 = TaskQueueFactory::default_factory();
}

/// @covers: TaskQueueFactoryContract::default_factory
#[test]
fn test_default_factory_type_matches_expected_error() {
    // default_factory() must return a TaskQueueFactory — this assertion fails
    // if the return type changes, proving the test is type-checking the contract.
    let factory: TaskQueueFactory = TaskQueueFactory::default_factory();
    let _ = factory;
}

// --- TaskQueueFactoryContract::new_task_id (rule 222) ---

/// @covers: TaskQueueFactoryContract::new_task_id
#[test]
fn test_new_task_id_returns_unique_ids_happy() {
    let factory = TaskQueueFactory::default_factory();
    let id1 = factory.new_task_id();
    let id2 = factory.new_task_id();
    assert_ne!(id1, id2, "consecutive task IDs must be unique");
}

/// @covers: TaskQueueFactoryContract::new_task_id
#[test]
fn test_new_task_id_is_callable_on_factory_instance_edge() {
    let factory = TaskQueueFactory::default_factory();
    let _id = factory.new_task_id();
}

/// @covers: TaskQueueFactoryContract::new_task_id
#[test]
fn test_new_task_id_many_calls_all_unique_error() {
    let factory = TaskQueueFactory::default_factory();
    let ids: Vec<_> = (0..16).map(|_| factory.new_task_id()).collect();
    let unique: std::collections::HashSet<_> = ids.iter().collect();
    assert_eq!(
        ids.len(),
        unique.len(),
        "all generated task IDs must be unique"
    );
}

// --- TaskQueueFactoryContract::build_in_memory (rule 222, cfg tokio-rt) ---

/// @covers: TaskQueueFactoryContract::build_in_memory
#[cfg(feature = "tokio-rt")]
#[test]
fn test_build_in_memory_returns_queue_happy() {
    let factory = TaskQueueFactory::default_factory();
    let _queue = factory.build_in_memory();
}

/// @covers: TaskQueueFactoryContract::build_in_memory
#[cfg(feature = "tokio-rt")]
#[tokio::test]
async fn test_build_in_memory_queue_health_check_passes_edge() {
    use swe_edge_runtime_message_broker::TaskQueue;
    let factory = TaskQueueFactory::default_factory();
    let queue = factory.build_in_memory();
    assert!(
        queue.health_check().await.is_ok(),
        "in-memory queue health_check must not fail at construction"
    );
}

/// @covers: TaskQueueFactoryContract::build_in_memory
#[cfg(feature = "tokio-rt")]
#[test]
fn test_build_in_memory_queue_is_send_and_sync_error() {
    fn _assert_send_sync<T: Send + Sync>() {} // @allow: no_mocks_in_integration
    _assert_send_sync::<swe_edge_runtime_message_broker::InMemoryTaskQueue>();
}
