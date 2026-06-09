//! Config theme — layered config loading, override deserialization, and errors.
//!
//! Observability types are re-exported from `swe-edge-observ-config`.

pub(crate) mod errors;
pub(crate) mod loader;
pub(crate) mod traits;
pub(crate) mod types;

pub use errors::ConfigError;
pub use swe_edge_observ_config::{ObservabilityConfig, TracingConfig};
pub(crate) use types::ConfigOverride;
