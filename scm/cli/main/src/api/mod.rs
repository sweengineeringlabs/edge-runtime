//! Public CLI contract surface.

mod error;
mod noop;
mod traits;
mod types;

pub use error::CliError;
pub use noop::{NoopCliCommand, NoopCliRunner, NoopValidator};
pub use traits::{CliCommand, CliRunner, Validator};
pub use types::{CliArgs, CliOutput};
