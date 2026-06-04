//! `ProfileResolver` api interface — contract for resolving profile specs.

pub mod profile_resolver_contract;
#[allow(clippy::module_inception)]
pub mod resolver;
pub use profile_resolver_contract::ProfileResolverContract;
#[expect(
    unused_imports,
    reason = "SEA api/ anchor — Resolver trait exported for consumers, not used internally"
)]
pub use resolver::Resolver;
