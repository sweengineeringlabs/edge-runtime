//! Integration tests for Runtime egress factory methods.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use swe_edge_egress_http::default_http_egress;
use swe_edge_runtime::{Egress, Runtime};

/// @covers: Runtime::http_egress
#[test]
fn test_http_egress_has_no_grpc_by_default() {
    let http = std::sync::Arc::new(default_http_egress().expect("http outbound"));
    let egress = Runtime::http_egress(http);
    assert!(egress.grpc().is_none());
}

/// @covers: Runtime::http_egress
#[test]
fn test_http_egress_http_returns_client() {
    let http = std::sync::Arc::new(default_http_egress().expect("http outbound"));
    let egress = Runtime::http_egress(http);
    let _ = egress.http();
}
