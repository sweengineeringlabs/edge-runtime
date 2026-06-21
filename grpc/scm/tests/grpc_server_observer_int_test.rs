//! Integration tests for the GrpcServerObserver trait.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use swe_edge_runtime_grpc::{GrpcServerObserver, NoopGrpcIngress, TonicGrpcServer};

fn server() -> TonicGrpcServer {
    TonicGrpcServer::new("127.0.0.1:0", NoopGrpcIngress::create())
}

#[test]
fn test_is_reflection_enabled_returns_false_on_new_server() {
    assert!(!GrpcServerObserver::is_reflection_enabled(&server()));
}

#[test]
fn test_is_reflection_enabled_returns_true_after_enable_reflection() {
    let s = server().enable_reflection(true);
    assert!(GrpcServerObserver::is_reflection_enabled(&s));
}

#[test]
fn test_is_reflection_enabled_returns_false_after_toggle_off() {
    let s = server().enable_reflection(true).enable_reflection(false);
    assert!(!GrpcServerObserver::is_reflection_enabled(&s));
}

#[test]
fn test_health_service_returns_some_on_new_server() {
    assert!(GrpcServerObserver::health_service(&server()).is_some());
}

#[test]
fn test_health_service_returns_none_after_without_health_service() {
    let s = server().without_health_service();
    assert!(GrpcServerObserver::health_service(&s).is_none());
}
