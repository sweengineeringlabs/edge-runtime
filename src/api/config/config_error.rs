//! `ConfigError` — errors from loading daemon configuration.

use thiserror::Error;

/// Errors that can occur when loading daemon configuration.
#[derive(Debug, Error)]
pub enum ConfigError {
    /// TOML or JSON parse error.
    #[error("config parse error: {0}")]
    Parse(String),
    /// File or directory I/O error.
    #[error("config io error: {0}")]
    Io(String),
    /// Requested tenant ID does not exist in the config.
    #[error("unknown tenant: '{0}'")]
    UnknownTenant(String),
    /// Tenant ID contains characters outside `[a-zA-Z0-9_-]`.
    #[error("invalid tenant id: '{0}' — only [a-zA-Z0-9_-] are allowed")]
    InvalidTenantId(String),
    /// Required environment variable is missing or malformed.
    #[error("invalid env var: {0}")]
    BadEnvVar(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_error_display_parse() {
        let e = ConfigError::Parse("bad toml".into());
        assert!(e.to_string().contains("parse error"));
    }

    #[test]
    fn test_config_error_display_unknown_tenant() {
        let e = ConfigError::UnknownTenant("ghost".into());
        assert!(e.to_string().contains("ghost"));
    }

    #[test]
    fn test_config_error_display_invalid_tenant_id() {
        let e = ConfigError::InvalidTenantId("../../etc".into());
        assert!(e.to_string().contains("[a-zA-Z0-9_-]"));
    }

    #[test]
    fn test_config_error_display_bad_env_var() {
        let e = ConfigError::BadEnvVar("SWE_EDGE_SHUTDOWN_TIMEOUT: expected integer".into());
        assert!(e.to_string().contains("SWE_EDGE_SHUTDOWN_TIMEOUT"));
    }
}
