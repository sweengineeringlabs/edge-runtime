//! Config error type and partial-override deserialization.
//! Observability types are re-exported from swe-edge-observ-config.

pub(crate) mod config_error;
pub(crate) mod config_override;

pub use config_error::ConfigError;
pub(crate) use config_override::ConfigOverride;
pub use swe_edge_observ_config::{ObservabilityConfig, TracingConfig};
