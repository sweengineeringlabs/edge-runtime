//! Integration tests for the daemon runner (run_until_signal).

use swe_edge_runtime::{runtime_manager, RuntimeConfig, DefaultInput, DefaultOutput};
use edge_proxy::new_null_lifecycle_monitor;
use swe_edge_egress_http::default_http_outbound;
use std::sync::Arc;

/// @covers: runtime_manager
#[test]
fn test_runtime_manager_builds_without_panic() {
    let http    = Arc::new(default_http_outbound().expect("http outbound"));
    let input   = Arc::new(DefaultInput::empty());
    let output  = Arc::new(DefaultOutput::new_http(http));
    let lc      = new_null_lifecycle_monitor();
    let _mgr    = runtime_manager(RuntimeConfig::default(), input, output, lc);
}
