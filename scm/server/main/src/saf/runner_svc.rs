//! SAF — `Runner` public service surface.
pub use crate::api::runtime::traits::runner::Runner;
pub use crate::api::runtime::types::runtime_builder_serve::RuntimeBuilderServe;
/// Identifies the `Runner` SAF contract in this crate.
pub const RUNNER_SVC: &str = "runner";
