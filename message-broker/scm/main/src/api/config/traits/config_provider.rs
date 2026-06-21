//! [`ConfigProvider`] — contract for types that provide application configuration.

use crate::api::config::types::application_config::ApplicationConfig;
use crate::api::config::types::broker_backend_config::BrokerBackendConfig;

/// Contract for types that provide message broker application configuration.
///
/// Implementors return the top-level [`ApplicationConfig`] and expose
/// its [`BrokerBackendConfig`] section for backend selection.
pub trait ConfigProvider {
    /// Return the full application configuration.
    fn application_config(&self) -> &ApplicationConfig;

    /// Return the broker backend configuration section.
    fn broker_backend_config(&self) -> &BrokerBackendConfig;
}
