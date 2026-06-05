//! Integration tests for Runtime egress factory methods.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::sync::Arc;
use swe_edge_egress_http::HttpTransportSvc;
use swe_edge_runtime::{Egress, Runtime};

/// @covers: Runtime::http_egress
#[test]
fn test_http_egress_has_no_grpc_by_default() {
    let http = Arc::from(HttpTransportSvc::default_http_egress().expect("http outbound"));
    let egress = Runtime::http_egress(http);
    assert!(egress.grpc().is_none());
}

/// @covers: Runtime::http_egress
#[test]
fn test_http_egress_http_returns_client() {
    let http = Arc::from(HttpTransportSvc::default_http_egress().expect("http outbound"));
    let egress = Runtime::http_egress(http);
    let _ = egress.http();
}
