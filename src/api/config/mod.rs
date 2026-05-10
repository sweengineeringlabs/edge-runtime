//! Config error type and partial-override deserialization.

pub(crate) mod config_error;
pub(crate) mod config_override;

pub use config_error::ConfigError;
pub(crate) use config_override::ConfigOverride;
