//! Integration tests for [`SchedulerError`].

use swe_edge_runtime_scheduler::SchedulerError;

/// @covers: SchedulerError::StartFailed
#[test]
fn test_scheduler_error_display_includes_message() {
    let e = SchedulerError::StartFailed("no threads".into());
    assert!(e.to_string().contains("no threads"));
}

/// @covers: SchedulerError
#[test]
fn test_scheduler_error_is_debug() {
    let e = SchedulerError::StartFailed("test".into());
    let debug = format!("{e:?}");
    assert!(!debug.is_empty());
}
