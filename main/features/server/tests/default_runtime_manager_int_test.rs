//! Integration tests for DefaultRuntimeManager.

use edge_proxy::new_null_lifecycle_monitor;
use std::sync::Arc;
use swe_edge_egress_http::default_http_egress;
use swe_edge_runtime::{runtime_manager, DefaultEgress, DefaultIngress, RuntimeConfig};

/// @covers: DefaultRuntimeManager
#[test]
fn test_default_runtime_manager_constructs_from_factory() {
    let http = Arc::new(default_http_egress().expect("http outbound"));
    let ingress = Arc::new(DefaultIngress::empty());
    let egress = Arc::new(DefaultEgress::new_http(http));
    let lc = new_null_lifecycle_monitor();
    let _mgr = runtime_manager(RuntimeConfig::default(), ingress, egress, lc);
}
