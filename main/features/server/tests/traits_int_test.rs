//! Integration tests for Validator trait.

use swe_edge_runtime::{load_config, validate_config};

/// @covers: traits
#[test]
fn test_validator_trait_validate_config_returns_ok_for_defaults() {
    let cfg = load_config().expect("load_config");
    assert!(validate_config(&cfg).is_ok());
}
