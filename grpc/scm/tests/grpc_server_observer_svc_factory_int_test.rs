//! Integration tests for GrpcServerObserverSvc factory.
//! @covers: GrpcServerObserverSvc::is_reflection_enabled
#![allow(clippy::unwrap_used, clippy::expect_used)]

use swe_edge_runtime_grpc::{
    GrpcServerObserver, GrpcServerObserverSvc, NoopGrpcIngress, TonicGrpcServer,
};

fn server() -> TonicGrpcServer {
    TonicGrpcServer::new("127.0.0.1:0", NoopGrpcIngress::create())
}

// ── is_reflection_enabled ─────────────────────────────────────────────────────

#[test]
fn test_is_reflection_enabled_default_is_false_happy() {
    // @covers: GrpcServerObserverSvc::is_reflection_enabled
    let s = server();
    assert!(!GrpcServerObserverSvc::is_reflection_enabled(&s));
}

#[test]
fn test_is_reflection_enabled_after_disable_returns_false_error() {
    // @covers: GrpcServerObserverSvc::is_reflection_enabled
    // Disabling after enabling must return false.
    let s = server().enable_reflection(true).enable_reflection(false);
    assert!(!GrpcServerObserverSvc::is_reflection_enabled(&s));
}

#[test]
fn test_is_reflection_enabled_after_enable_is_true_edge() {
    // @covers: GrpcServerObserverSvc::is_reflection_enabled
    let s = server().enable_reflection(true);
    assert!(GrpcServerObserverSvc::is_reflection_enabled(&s));
}

// ── GrpcServerObserver::health_service ────────────────────────────────────────

#[test]
fn test_health_service_default_returns_some_happy() {
    // @covers: GrpcServerObserver::health_service
    let s = server();
    assert!(GrpcServerObserver::health_service(&s).is_some());
    // Negative: after removal, must be absent
    let s2 = s.without_health_service();
    assert!(GrpcServerObserver::health_service(&s2).is_none());
}

#[test]
fn test_health_service_after_remove_returns_none_error() {
    // @covers: GrpcServerObserver::health_service
    let s = server().without_health_service();
    assert!(GrpcServerObserver::health_service(&s).is_none());
}

#[test]
fn test_health_service_custom_replaces_default_edge() {
    // @covers: GrpcServerObserver::health_service
    use std::sync::Arc;
    use swe_edge_runtime_grpc::HealthService;
    let custom = Arc::new(HealthService::new());
    let ptr = Arc::as_ptr(&custom);
    let s = server().with_health_service(custom);
    assert_eq!(
        Arc::as_ptr(GrpcServerObserver::health_service(&s).unwrap()),
        ptr
    );
}
