//! Integration tests for the daemon runner (run_until_signal).

use edge_proxy::new_null_lifecycle_monitor;
use std::sync::Arc;
use swe_edge_egress_http::default_http_egress;
use swe_edge_runtime::{runtime_manager, Runtime, RuntimeConfig};

/// @covers: runtime_manager
#[test]
fn test_runtime_manager_builds_without_panic() {
    let http = Arc::new(default_http_egress().expect("http outbound"));
    let ingress = Arc::new(Runtime::empty_ingress());
    let egress = Arc::new(Runtime::http_egress(http));
    let lc = new_null_lifecycle_monitor();
    let _mgr = runtime_manager(RuntimeConfig::default(), ingress, egress, lc);
}
