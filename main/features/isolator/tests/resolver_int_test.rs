//! Integration tests for the profile resolver.

use swe_edge_egress_subprocess::IsolationError;
use swe_edge_runtime_isolator::{IsolatorConfig, IsolatorSvc};

/// @covers: ProfileResolver::resolve — noop resolves successfully.
#[test]
fn test_resolver_resolves_noop_profile() {
    let registry = IsolatorSvc::build_registry(IsolatorConfig::default()).expect("registry build");
    let profile = registry.get("noop").unwrap();
    assert_eq!(profile.name(), "noop");
}

/// @covers: ProfileResolver::resolve — unknown kind returns error.
#[test]
fn test_resolver_unknown_kind_returns_error() {
    let toml = r#"
[profiles.bad]
kind = "nonexistent_kind"
"#;
    let config: IsolatorConfig = toml::from_str(toml).expect("valid toml");
    let err = IsolatorSvc::build_registry(config).unwrap_err();
    assert!(matches!(err, IsolationError::UnknownProfile { .. }));
}
