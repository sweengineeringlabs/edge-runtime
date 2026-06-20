//! Service Abstraction Framework — the only public export surface for consumers.

mod cli;
mod validator_svc;

pub use cli::{CliArgs, CliCommand, CliError, CliOutput, CliRunner, NoopCliCommand, NoopCliRunner};
pub use validator_svc::{NoopValidator, Validator};
