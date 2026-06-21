//! Integration tests for the grpc_load_monitor_svc SAF surface.

use swe_edge_runtime::{GrpcLoadMonitor, GRPC_LOAD_MONITOR_SVC};

/// @covers: GRPC_LOAD_MONITOR_SVC
#[test]
fn test_grpc_load_monitor_svc_slug_is_correct_happy() {
    assert_eq!(GRPC_LOAD_MONITOR_SVC, "grpc_load_monitor");
}

#[test]
fn test_grpc_load_monitor_svc_slug_is_non_empty_error() {
    assert!(!GRPC_LOAD_MONITOR_SVC.is_empty());
}

#[test]
fn test_grpc_load_monitor_is_object_safe_edge() {
    fn _accept(_: &dyn GrpcLoadMonitor) {}
}
