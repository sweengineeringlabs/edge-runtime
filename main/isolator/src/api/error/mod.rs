//! Error types for swe_edge_runtime_isolator.

#[allow(clippy::module_inception)]
pub mod error;
pub mod isolator_error;

pub use error::Error;
#[expect(
    unused_imports,
    reason = "SEA api/ anchor — IsolatorError alias exported for consumers, not used internally"
)]
pub use isolator_error::IsolatorError;
