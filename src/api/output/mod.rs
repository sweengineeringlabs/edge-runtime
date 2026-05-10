//! Outbound gateway contract.

pub(crate) mod default_output;
#[allow(clippy::module_inception)]
pub(crate) mod output;

pub use default_output::DefaultOutput;
pub use output::Output;
