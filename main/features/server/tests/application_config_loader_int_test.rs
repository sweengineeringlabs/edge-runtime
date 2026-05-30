//! Integration tests for ApplicationConfigLoader.

use swe_edge_runtime::ServerConfigLoader;

/// @covers: ApplicationConfigLoader
#[test]
fn test_application_config_loader_loads_default_config() {
    let cfg = ServerConfigLoader::load_config().expect("load_config");
    assert!(!cfg.http_bind.is_empty());
    assert!(!cfg.grpc_bind.is_empty());
}
