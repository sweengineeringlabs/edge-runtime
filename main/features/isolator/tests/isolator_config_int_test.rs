//! Integration tests for IsolatorConfig.

use swe_edge_runtime_isolator::IsolatorConfig;

/// @covers: IsolatorConfig::section_name
#[test]
fn test_isolator_config_section_name_is_subprocess_policy() {
    use swe_edge_configbuilder::ConfigSection as _;
    assert_eq!(IsolatorConfig::section_name(), "subprocess_policy");
}

/// @covers: IsolatorConfig::default
#[test]
fn test_isolator_config_default_has_default_noop_profile() {
    let cfg = IsolatorConfig::default();
    let profile = cfg.profiles.get("default").expect("default profile");
    assert_eq!(profile.kind, "noop");
}
