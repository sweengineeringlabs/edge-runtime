//! Service Abstraction Framework — the only public export surface for consumers.

mod cli;
mod validator_svc;

pub use crate::api::CliArgs;
pub use cli::{CliCommand, CliError, CliOutput, CliRunner, NoopCliCommand, NoopCliRunner};
pub use validator_svc::{NoopValidator, Validator};
