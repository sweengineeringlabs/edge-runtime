//! Concrete isolation profile implementations.

pub(crate) mod noop;
pub(crate) mod profile;

#[cfg(all(target_os = "linux", feature = "seccomp"))]
pub(crate) mod seccomp;

pub(crate) mod swe;
#[expect(
    unused_imports,
    reason = "SEA core/ anchor — used when SAF factory is integrated"
)]
pub(crate) use swe::DefaultSweEdgeRuntimeIsolator;
