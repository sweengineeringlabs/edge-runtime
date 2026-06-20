//! Service Abstraction Framework — the only public export surface for consumers.

mod cli_runner_svc;
mod validator_svc;

pub use cli_runner_svc::*;
pub use validator_svc::*;
