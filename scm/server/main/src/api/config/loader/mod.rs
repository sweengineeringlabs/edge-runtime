//! Config loader interface — mirrors `core/config/loader`.

pub mod application_config_loader;

pub use crate::api::config::traits::loader::config_loader::ConfigLoader;
pub use application_config_loader::ApplicationConfigLoader;
