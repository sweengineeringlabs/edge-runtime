//! Integration tests for RuntimeResult type alias.

use swe_edge_runtime::{RuntimeError, RuntimeResult};

/// @covers: RuntimeResult
#[test]
fn test_runtime_result_ok_holds_value() {
    let r: RuntimeResult<u32> = Ok(42);
    assert_eq!(r.unwrap(), 42);
}

/// @covers: RuntimeResult
#[test]
fn test_runtime_result_err_holds_error() {
    let r: RuntimeResult<u32> = Err(RuntimeError::StartFailed("test".into()));
    assert!(r.is_err());
}
