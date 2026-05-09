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

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: load_config — returns defaults when no application.toml present
    #[test]
    fn test_load_config_returns_ok_with_no_toml() {
        let result = load_config();
        assert!(result.is_ok(), "load_config failed: {result:?}");
    }

    /// @covers: load_config_from — accepts valid directory
    #[test]
    fn test_load_config_from_accepts_temp_dir() {
        let dir = tempfile::tempdir().unwrap();
        let result = load_config_from(dir.path());
        assert!(result.is_ok());
    }

    /// @covers: load_config_xdg — returns ok for non-existent app
    #[test]
    fn test_load_config_xdg_returns_ok_for_unknown_app() {
        let result = load_config_xdg("swe-edge-test-nonexistent-xyz");
        assert!(result.is_ok());
    }
}
