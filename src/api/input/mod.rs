//! Inbound gateway contract.

pub(crate) mod default_input;
#[allow(clippy::module_inception)]
pub(crate) mod input;

pub use default_input::DefaultInput;
pub use input::Input;
