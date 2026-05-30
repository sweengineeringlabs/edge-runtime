//! `SeccompIsolator` api interface — seccomp-bpf isolation profile contract.

pub mod seccomp_isolation_profile;
pub mod seccomp_isolator;
pub use seccomp_isolation_profile::SeccompIsolationProfile;
pub use seccomp_isolator::SeccompIsolator;
