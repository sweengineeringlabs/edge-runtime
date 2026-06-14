//! SAF — `ConfigLoader` public service surface.
pub use crate::api::config::traits::config_loader::ConfigLoader;
/// Identifies the `ConfigLoader` SAF contract in this crate.
pub const CONFIG_LOADER_SVC: &str = "config_loader";
