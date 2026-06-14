//! Integration tests for the config_loader_svc SAF surface.
#![allow(clippy::unwrap_used)]

use swe_edge_runtime::{ConfigError, ConfigLoader, RuntimeConfig, CONFIG_LOADER_SVC};

struct OkLoader;

impl ConfigLoader for OkLoader {
    fn load(&self) -> Result<RuntimeConfig, ConfigError> {
        Ok(RuntimeConfig::default())
    }
    fn load_for_tenant(&self, tenant_id: &str) -> Result<RuntimeConfig, ConfigError> {
        if tenant_id.is_empty() {
            Err(ConfigError::UnknownTenant(String::new()))
        } else {
            Ok(RuntimeConfig::default())
        }
    }
    fn load_section<T>(&self, _key: &str) -> Result<T, ConfigError>
    where
        T: serde::de::DeserializeOwned + Default,
    {
        Ok(T::default())
    }
}

struct ErrLoader;

impl ConfigLoader for ErrLoader {
    fn load(&self) -> Result<RuntimeConfig, ConfigError> {
        Err(ConfigError::Io("test io error".into()))
    }
    fn load_for_tenant(&self, tenant_id: &str) -> Result<RuntimeConfig, ConfigError> {
        Err(ConfigError::UnknownTenant(tenant_id.into()))
    }
    fn load_section<T>(&self, _key: &str) -> Result<T, ConfigError>
    where
        T: serde::de::DeserializeOwned + Default,
    {
        Err(ConfigError::Parse("bad section".into()))
    }
}

/// @covers: CONFIG_LOADER_SVC
#[test]
fn test_config_loader_svc_slug_is_correct_happy() {
    assert_eq!(CONFIG_LOADER_SVC, "config_loader");
}

// ── ConfigLoader::load ────────────────────────────────────────────────────────

#[test]
fn test_load_ok_loader_returns_valid_runtime_config_happy() {
    let result = OkLoader.load();
    assert!(result.is_ok());
    assert!(!result.unwrap().http_bind.is_empty());
}

#[test]
fn test_load_err_loader_returns_io_error_edge() {
    let result = ErrLoader.load();
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), ConfigError::Io(_)));
}

// ── ConfigLoader::load_for_tenant ─────────────────────────────────────────────

#[test]
fn test_load_for_tenant_with_non_empty_id_returns_config_happy() {
    let result = OkLoader.load_for_tenant("tenant-1");
    assert!(result.is_ok());
}

#[test]
fn test_load_for_tenant_with_empty_id_returns_unknown_tenant_error() {
    let result = OkLoader.load_for_tenant("");
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), ConfigError::UnknownTenant(_)));
}

#[test]
fn test_load_for_tenant_err_loader_always_returns_error_edge() {
    let result = ErrLoader.load_for_tenant("any");
    assert!(result.is_err());
}

// ── ConfigLoader::load_section ────────────────────────────────────────────────

#[test]
fn test_load_section_ok_loader_returns_default_for_absent_key_happy() {
    let result: Result<i64, _> = OkLoader.load_section("missing.key");
    assert_eq!(result.unwrap(), 0);
}

#[test]
fn test_load_section_err_loader_returns_parse_error_error() {
    let result: Result<i64, _> = ErrLoader.load_section("bad.section");
    assert!(matches!(result.unwrap_err(), ConfigError::Parse(_)));
}

#[test]
fn test_load_section_bool_default_is_false_edge() {
    let result: Result<bool, _> = OkLoader.load_section("any.key");
    assert!(!result.unwrap());
}
