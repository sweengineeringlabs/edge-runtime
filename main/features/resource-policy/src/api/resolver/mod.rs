//! Resolver api interface — resource limits resolver contract.

pub mod resource_limits_resolver;
#[expect(
    unused_imports,
    reason = "SEA api/ anchor — ResourceLimitsResolver exported for consumers, not used internally"
)]
pub use resource_limits_resolver::ResourceLimitsResolver;
