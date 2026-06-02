//! Policy api interface — runner contract.

pub mod runner;
#[expect(
    unused_imports,
    reason = "SEA api/ anchor — ResourcePolicyRunner exported for consumers, not used internally"
)]
pub use runner::ResourcePolicyRunner;
