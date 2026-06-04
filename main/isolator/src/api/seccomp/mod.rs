//! `SeccompIsolator` api interface — seccomp-bpf isolation profile contract.

pub mod seccomp_isolation_profile;
pub mod seccomp_isolator;
#[expect(
    unused_imports,
    reason = "SEA api/ anchor — SeccompIsolationProfile trait exported for consumers, not used internally"
)]
pub use seccomp_isolation_profile::SeccompIsolationProfile;
#[expect(
    unused_imports,
    reason = "SEA api/ anchor — SeccompIsolator trait exported for consumers, not used internally"
)]
pub use seccomp_isolator::SeccompIsolator;
