//! SAF — [`ConfigProvider`] public service surface.
//!
//! Exposes the [`ConfigProvider`] trait and its associated configuration types
//! for consumers that need to read message broker application configuration.

pub use crate::api::config::traits::config_provider::ConfigProvider;
pub use crate::api::config::types::application_config::ApplicationConfig;
pub use crate::api::config::types::broker_backend_config::BrokerBackendConfig;

/// TOML section name under which message-broker configuration is nested.
pub const BROKER_CONFIG_SECTION: &str = "message_broker";
