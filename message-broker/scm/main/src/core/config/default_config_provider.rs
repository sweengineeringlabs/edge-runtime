//! Default configuration values for the message broker.
#![allow(dead_code)]

use crate::api::ApplicationConfig;
use crate::api::BrokerBackendConfig;
use crate::api::ConfigProvider;

/// Default implementation of [`ConfigProvider`] backed by a stored [`ApplicationConfig`].
///
/// Used as the baseline configuration when no external source is available.
/// Seeded with the compile-time defaults from [`ApplicationConfig::default`].
pub(crate) struct DefaultConfigProvider {
    config: ApplicationConfig,
}

impl DefaultConfigProvider {
    /// Create a new provider from the given configuration.
    pub(crate) fn new(config: ApplicationConfig) -> Self {
        Self { config }
    }

    /// Return the default broker backend identifier (`"inmemory"`).
    pub(crate) fn default_backend() -> &'static str {
        DEFAULT_BACKEND_KIND
    }
}

impl ConfigProvider for DefaultConfigProvider {
    fn application_config(&self) -> &ApplicationConfig {
        &self.config
    }

    fn broker_backend_config(&self) -> &BrokerBackendConfig {
        &self.config.message_broker
    }
}

/// Default backend identifier matching [`BrokerBackendConfig`]'s default value.
pub(crate) const DEFAULT_BACKEND_KIND: &str = "inmemory";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_stores_config_and_returns_backend_default() {
        let provider = DefaultConfigProvider::new(ApplicationConfig::default());
        assert_eq!(provider.broker_backend_config().backend, "inmemory");
    }

    #[test]
    fn test_application_config_returns_same_instance() {
        let provider = DefaultConfigProvider::new(ApplicationConfig::default());
        assert_eq!(
            provider.application_config().message_broker.backend,
            "inmemory"
        );
    }

    #[test]
    fn test_default_backend_matches_config_default() {
        assert_eq!(
            DefaultConfigProvider::default_backend(),
            DEFAULT_BACKEND_KIND,
        );
    }
}
