//! SAF — `RuntimeManager` public service surface.
pub use crate::api::runtime::traits::runtime_manager::RuntimeManager;
/// Identifies the `RuntimeManager` SAF contract in this crate.
pub const RUNTIME_MANAGER_SVC: &str = "runtime_manager";
