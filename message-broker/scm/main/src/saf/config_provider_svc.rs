//! SAF — [`ConfigProvider`] public service surface.
//!
//! Exposes the [`ConfigProvider`] trait and its associated configuration types
//! for consumers that need to read message broker application configuration.

pub use crate::api::ApplicationConfig;
pub use crate::api::BrokerBackendConfig;
pub use crate::api::ConfigProvider;

/// TOML section name under which message-broker configuration is nested.
pub const BROKER_CONFIG_SECTION: &str = "message_broker";
