//! Public-API integration tests for saf daemon methods on Runtime.

use edge_proxy::new_null_lifecycle_monitor;
use std::sync::Arc;
use swe_edge_egress_http::default_http_egress;
use swe_edge_runtime::{Runtime, RuntimeConfig};

/// @covers: runtime_manager
#[test]
fn test_runtime_manager_factory_constructs_without_panic() {
    let http = Arc::new(default_http_egress().expect("http outbound"));
    let ingress = Arc::new(Runtime::empty_ingress());
    let egress = Arc::new(Runtime::http_egress(http));
    let lc = new_null_lifecycle_monitor();
    let _mgr = Runtime::runtime_manager(RuntimeConfig::default(), ingress, egress, lc);
}
