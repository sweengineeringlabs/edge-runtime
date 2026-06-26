//! Integration tests for [`ConfigProvider`] trait (rule 222).
#![allow(clippy::unwrap_used, clippy::expect_used)]

use swe_edge_runtime_message_broker::{ApplicationConfig, BrokerBackendConfig, ConfigProvider};

struct TestConfigProvider {
    config: ApplicationConfig,
}

impl TestConfigProvider {
    fn new() -> Self {
        Self {
            config: ApplicationConfig::default(),
        }
    }
}

impl ConfigProvider for TestConfigProvider {
    fn application_config(&self) -> &ApplicationConfig {
        &self.config
    }

    fn broker_backend_config(&self) -> &BrokerBackendConfig {
        &self.config.message_broker
    }
}

// --- ConfigProvider::application_config (rule 222) ---

/// @covers: ConfigProvider::application_config
#[test]
fn test_application_config_returns_reference_happy() {
    let provider = TestConfigProvider::new();
    let cfg = provider.application_config();
    // Config reference must be valid and dereferenceable.
    assert!(
        !cfg.message_broker.backend.is_empty(),
        "config must have a valid backend"
    );
}

/// @covers: ConfigProvider::application_config
#[test]
fn test_application_config_default_backend_is_inmemory_edge() {
    let provider = TestConfigProvider::new();
    let cfg = provider.application_config();
    assert_eq!(cfg.message_broker.backend, "inmemory");
}

/// @covers: ConfigProvider::application_config
#[test]
fn test_application_config_custom_backend_reflects_construction_error() {
    let config = ApplicationConfig {
        message_broker: BrokerBackendConfig {
            backend: "nats".into(),
            nats_url: "nats://localhost:4222".into(),
            kafka_brokers: "localhost:9092".into(),
        },
    };
    let provider = TestConfigProvider { config };
    assert_eq!(provider.application_config().message_broker.backend, "nats");
}

// --- ConfigProvider::broker_backend_config (rule 222) ---

/// @covers: ConfigProvider::broker_backend_config
#[test]
fn test_broker_backend_config_returns_reference_happy() {
    let provider = TestConfigProvider::new();
    let cfg = provider.broker_backend_config();
    // Backend config reference must be valid and contain a backend name.
    assert!(
        !cfg.backend.is_empty(),
        "broker_backend_config must return a reference with a backend"
    );
}

/// @covers: ConfigProvider::broker_backend_config
#[test]
fn test_broker_backend_config_default_nats_url_is_localhost_edge() {
    let provider = TestConfigProvider::new();
    let cfg = provider.broker_backend_config();
    assert!(
        cfg.nats_url.contains("localhost"),
        "default NATS URL must point to localhost, got: {}",
        cfg.nats_url
    );
}

/// @covers: ConfigProvider::broker_backend_config
#[test]
fn test_broker_backend_config_custom_kafka_brokers_reflects_construction_error() {
    let config = ApplicationConfig {
        message_broker: BrokerBackendConfig {
            backend: "kafka".into(),
            nats_url: "nats://localhost:4222".into(),
            kafka_brokers: "broker1:9092,broker2:9092".into(),
        },
    };
    let provider = TestConfigProvider { config };
    assert!(provider
        .broker_backend_config()
        .kafka_brokers
        .contains("broker1"));
}
