//! Application config loader interface — mirrors `core/config/loader/application_config_loader`.

pub use crate::api::config::traits::application_config_loader::ApplicationConfigLoader;

/// Maximum number of config layers that can be stacked by the loader chain.
pub const MAX_CONFIG_LAYERS: usize = 4;
