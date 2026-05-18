//! Integration tests verifying swe-edge-observ-config is wired correctly through
//! the runtime SAF (load_section, TracingConfig, ObservabilityConfig).

use swe_edge_runtime::{
    load_section, load_section_from, ObservabilityConfig, TracingConfig, TracingFormat,
    TracingLevel,
};

/// @covers: load_section
#[test]
fn test_observ_config_int_load_section_returns_tracing_defaults() {
    let cfg: TracingConfig = load_section("observability.tracing").unwrap();
    assert!(cfg.enabled);
    assert_eq!(cfg.level, TracingLevel::Info);
    assert_eq!(cfg.format, TracingFormat::Pretty);
}

/// @covers: load_section_from
#[test]
fn test_observ_config_int_load_section_from_temp_dir_returns_defaults() {
    let dir = tempfile::tempdir().unwrap();
    let cfg: TracingConfig = load_section_from("observability.tracing", dir.path()).unwrap();
    assert!(cfg.enabled);
}

/// @covers: load_section_from
#[test]
fn test_observ_config_int_load_section_from_applies_application_toml_override() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(
        dir.path().join("application.toml"),
        "[observability.tracing]\nenabled = false\nlevel = \"warn\"",
    )
    .unwrap();
    let cfg: TracingConfig = load_section_from("observability.tracing", dir.path()).unwrap();
    assert!(!cfg.enabled);
    assert_eq!(cfg.level, TracingLevel::Warn);
}

/// @covers: ObservabilityConfig
#[test]
fn test_observ_config_int_observability_config_deserializes_from_toml() {
    let toml = "[tracing]\nlevel = \"debug\"\nformat = \"json\"";
    let cfg: ObservabilityConfig = toml::from_str(toml).unwrap();
    assert_eq!(cfg.tracing.level, TracingLevel::Debug);
    assert_eq!(cfg.tracing.format, TracingFormat::Json);
}
