//! `ProfileResolver` api interface — contract for resolving profile specs to profiles.

/// Trait defining the profile resolution contract.
///
/// Implemented by `core::profile::resolver::ProfileResolver`.
#[expect(
    dead_code,
    reason = "SEA api/ anchor — exported for consumers, not used internally"
)]
pub trait ProfileResolverContract {}
