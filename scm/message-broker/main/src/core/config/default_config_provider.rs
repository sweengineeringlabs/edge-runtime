//! [`DefaultConfigProvider`] — default implementation of [`ConfigProvider`].

use crate::api::config::traits::config_provider::ConfigProvider;
use crate::api::config::types::application_config::ApplicationConfig;
use crate::api::config::types::broker_backend_config::BrokerBackendConfig;

pub(crate) struct DefaultConfigProvider {
    config: ApplicationConfig,
}

impl DefaultConfigProvider {
    pub(crate) fn new(config: ApplicationConfig) -> Self {
        Self { config }
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
}
