//! Integration tests for DefaultEgress.

use std::sync::Arc;
use swe_edge_egress_http::default_http_egress;
use swe_edge_runtime::{DefaultEgress, Egress};

/// @covers: DefaultEgress
#[test]
fn test_default_egress_new_http_has_no_grpc() {
    let http = Arc::new(default_http_egress().expect("http outbound"));
    let egress = DefaultEgress::new_http(http);
    assert!(egress.grpc().is_none());
}

/// @covers: DefaultEgress
#[test]
fn test_default_egress_http_returns_client() {
    let http = Arc::new(default_http_egress().expect("http outbound"));
    let egress = DefaultEgress::new_http(http);
    let _ = egress.http(); // type-checks the Egress impl
}
