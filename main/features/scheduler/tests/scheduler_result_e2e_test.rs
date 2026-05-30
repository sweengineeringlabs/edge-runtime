//! Integration tests for [`SchedulerResult`].

use swe_edge_runtime_scheduler::{SchedulerError, SchedulerResult};

/// @covers: SchedulerResult
#[test]
fn test_scheduler_result_ok_wraps_value() {
    let result: SchedulerResult<i32> = Ok(42);
    assert_eq!(result.unwrap(), 42);
}

/// @covers: SchedulerResult
#[test]
fn test_scheduler_result_err_wraps_error() {
    let result: SchedulerResult<i32> = Err(SchedulerError::StartFailed("fail".into()));
    assert!(result.is_err());
}
