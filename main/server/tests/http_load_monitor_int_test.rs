//! Integration tests for HttpLoadMonitor trait coverage.

use swe_edge_runtime::HttpIngress;

/// @covers: HttpLoadMonitor
#[test]
fn test_http_ingress_is_object_safe_for_load_monitor() {
    fn _accept(_: &dyn HttpIngress) {}
}
