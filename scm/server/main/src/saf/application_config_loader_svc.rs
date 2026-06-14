//! SAF — `ApplicationConfigLoader` public service surface.
pub use crate::api::config::traits::application_config_loader::ApplicationConfigLoader;
/// Identifies the `ApplicationConfigLoader` SAF contract in this crate.
pub const APPLICATION_CONFIG_LOADER_SVC: &str = "application_config_loader";
