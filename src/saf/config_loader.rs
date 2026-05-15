//! SAF — config-loading convenience functions.

use crate::api::config::ConfigError;
use crate::api::config_loader::ConfigLoader;
use crate::api::error::RuntimeError;
use crate::api::traits::Validator;
use crate::api::types::RuntimeConfig;
use crate::core::validator::ConfigValidator;
use crate::core::DefaultConfigLoader;

/// Load config using the default layered chain
/// (`default.toml` → `application.toml` → env vars).
///
/// The config directory is resolved from `SWE_EDGE_CONFIG_DIR` or
/// defaults to `config/` relative to the working directory.
/// Consumer apps should prefer [`load_config_from`] to supply their
/// own path explicitly.
pub fn load_config() -> Result<RuntimeConfig, ConfigError> {
    DefaultConfigLoader::new().load()
}

/// Load config from an explicit directory.
///
/// Identical layer chain to [`load_config`] but reads
/// `<dir>/application.toml` instead of relying on env or cwd.
pub fn load_config_from(dir: impl Into<std::path::PathBuf>) -> Result<RuntimeConfig, ConfigError> {
    DefaultConfigLoader::with_dir(dir).load()
}

/// Load config scoped to a tenant
/// (`default.toml` → `application.toml` → `tenants/<id>.toml` → env vars).
///
/// See [`load_tenant_config_from`] for the consumer-app variant.
pub fn load_tenant_config(tenant_id: &str) -> Result<RuntimeConfig, ConfigError> {
    DefaultConfigLoader::new().load_for_tenant(tenant_id)
}

/// Load tenant config from an explicit directory.
///
/// Reads `<dir>/application.toml` and `<dir>/tenants/<tenant_id>.toml`.
/// Intended for consumer apps that own their config directory layout.
pub fn load_tenant_config_from(
    tenant_id: &str,
    dir: impl Into<std::path::PathBuf>,
) -> Result<RuntimeConfig, ConfigError> {
    DefaultConfigLoader::with_dir(dir).load_for_tenant(tenant_id)
}

/// Load config following the XDG Base Directory specification.
///
/// Layer chain (last wins):
/// - `$XDG_CONFIG_DIRS/<app_name>/application.toml` (system-wide, default `/etc/xdg/`)
/// - `$XDG_CONFIG_HOME/<app_name>/application.toml` (user-level, default `~/.config/`)
/// - `$SWE_EDGE_CONFIG_DIR/application.toml` (explicit override, if set)
/// - `SWE_EDGE_*` environment variables (always top priority)
pub fn load_config_xdg(app_name: &str) -> Result<RuntimeConfig, ConfigError> {
    DefaultConfigLoader::xdg(app_name).load()
}

/// Load tenant config following the XDG Base Directory specification.
///
/// Same XDG layer chain as [`load_config_xdg`], with
/// `tenants/<tenant_id>.toml` applied on top at the highest-priority
/// directory where it exists.
pub fn load_tenant_config_xdg(
    app_name: &str,
    tenant_id: &str,
) -> Result<RuntimeConfig, ConfigError> {
    DefaultConfigLoader::xdg(app_name).load_for_tenant(tenant_id)
}

/// Validate a [`RuntimeConfig`] using the built-in [`ConfigValidator`].
///
/// Returns `Err(RuntimeError::StartFailed)` if any field is out of bounds.
pub fn validate_config(config: &RuntimeConfig) -> Result<(), RuntimeError> {
    ConfigValidator.validate(config)
}

/// Load an arbitrary TOML section from the default config chain.
///
/// `key` is a dotted path, e.g. `"observability.tracing"` or
/// `"application.completion"`.  Each config directory's `application.toml`
/// layers over the shipped `default.toml`.
///
/// ```rust,ignore
/// let tracing: TracingConfig = load_section("observability.tracing")?;
/// ```
pub fn load_section<T>(key: &str) -> Result<T, ConfigError>
where
    T: serde::de::DeserializeOwned + Default,
{
    DefaultConfigLoader::new().load_section(key)
}

/// Load an arbitrary TOML section from an explicit config directory.
pub fn load_section_from<T>(
    key: &str,
    dir: impl Into<std::path::PathBuf>,
) -> Result<T, ConfigError>
where
    T: serde::de::DeserializeOwned + Default,
{
    DefaultConfigLoader::with_dir(dir).load_section(key)
}

/// Load an arbitrary TOML section following the XDG Base Directory chain.
///
/// Layer order matches [`load_config_xdg`].
pub fn load_section_xdg<T>(app_name: &str, key: &str) -> Result<T, ConfigError>
where
    T: serde::de::DeserializeOwned + Default,
{
    DefaultConfigLoader::xdg(app_name).load_section(key)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: load_config
    #[test]
    fn test_load_config_returns_ok_with_no_toml() {
        let result = load_config();
        assert!(result.is_ok(), "load_config failed: {result:?}");
    }

    /// @covers: load_config_from
    #[test]
    fn test_load_config_from_accepts_temp_dir() {
        let dir = tempfile::tempdir().unwrap();
        let result = load_config_from(dir.path());
        assert!(result.is_ok());
    }

    /// @covers: load_config_xdg
    #[test]
    fn test_load_config_xdg_returns_ok_for_unknown_app() {
        let result = load_config_xdg("swe-edge-test-nonexistent-xyz");
        assert!(result.is_ok());
    }

    /// @covers: load_tenant_config
    #[test]
    fn test_load_tenant_config_unknown_returns_error() {
        assert!(load_tenant_config("nonexistent-tenant-xyz").is_err());
    }

    /// @covers: load_tenant_config_from
    #[test]
    fn test_load_tenant_config_from_missing_returns_error() {
        let dir = tempfile::tempdir().unwrap();
        assert!(load_tenant_config_from("no-such-tenant", dir.path()).is_err());
    }

    /// @covers: load_tenant_config_xdg
    #[test]
    fn test_load_tenant_config_xdg_missing_returns_error() {
        assert!(load_tenant_config_xdg("swe-edge-test-xyz", "no-tenant").is_err());
    }

    /// @covers: validate_config
    #[test]
    fn test_validate_config_accepts_defaults() {
        let cfg = load_config().unwrap();
        assert!(validate_config(&cfg).is_ok());
    }

    /// @covers: load_section
    #[test]
    fn test_load_section_returns_observability_tracing_defaults() {
        use crate::api::config::TracingConfig;
        use swe_edge_observ_config::TracingLevel;
        let cfg: TracingConfig = load_section("observability.tracing").unwrap();
        assert!(cfg.enabled);
        assert_eq!(cfg.level, TracingLevel::Info);
    }

    /// @covers: load_section_from
    #[test]
    fn test_load_section_from_reads_section_from_supplied_dir() {
        use crate::api::config::TracingConfig;
        use std::io::Write as _;
        let dir = tempfile::tempdir().unwrap();
        let mut f = std::fs::File::create(dir.path().join("application.toml")).unwrap();
        writeln!(f, "[observability.tracing]\nenabled = false").unwrap();
        let cfg: TracingConfig = load_section_from("observability.tracing", dir.path()).unwrap();
        assert!(!cfg.enabled);
    }

    /// @covers: load_section_xdg
    #[test]
    fn test_load_section_xdg_returns_defaults_for_unknown_app() {
        use crate::api::config::TracingConfig;
        let cfg: TracingConfig =
            load_section_xdg("swe-edge-test-nonexistent-xyz", "observability.tracing").unwrap();
        assert!(cfg.enabled);
    }
}
