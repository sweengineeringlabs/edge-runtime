//! `Profile` api interface — profile resolver contract.

pub mod resolver;
#[expect(
    unused_imports,
    reason = "SEA api/ anchor — ProfileResolverContract exported for consumers, not used internally"
)]
pub use resolver::ProfileResolverContract;
