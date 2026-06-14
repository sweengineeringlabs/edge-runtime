//! [`ServerConfigLoader`] — factory for loading runtime server configuration.

/// Factory for loading and validating [`RuntimeConfig`](crate::api::config::RuntimeConfig).
///
/// All config-loading methods live on this type rather than as free functions
/// so the SAF layer satisfies the OOP constraint (Rule 191).
pub struct ServerConfigLoader;
