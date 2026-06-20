//! Integration tests for [`swe_edge_runtime_grpc::GrpcHealthCheck`].

use swe_edge_runtime_grpc::GrpcHealthCheck;

/// @covers: GrpcHealthCheck::healthy — happy path
#[test]
fn test_healthy_flag_is_true_happy() {
    let hc = GrpcHealthCheck::healthy();
    assert!(hc.healthy);
    assert!(hc.message.is_none());
}

/// @covers: GrpcHealthCheck::unhealthy — sets flag false and message
#[test]
fn test_unhealthy_flag_is_false_and_message_set_error() {
    let hc = GrpcHealthCheck::unhealthy("connection refused");
    assert!(!hc.healthy);
    assert_eq!(hc.message.as_deref(), Some("connection refused"));
}

/// @covers: GrpcHealthCheck::unhealthy — empty message accepted
#[test]
fn test_unhealthy_empty_message_accepted_edge() {
    let hc = GrpcHealthCheck::unhealthy("");
    assert!(!hc.healthy);
    assert_eq!(hc.message.as_deref(), Some(""));
}
