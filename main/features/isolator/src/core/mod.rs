//! Concrete isolation profile implementations.

pub(crate) mod noop;
pub(crate) mod profile;

#[cfg(all(target_os = "linux", feature = "seccomp"))]
pub(crate) mod seccomp;

pub(crate) mod swe;
pub(crate) use swe::DefaultSweEdgeRuntimeIsolator;
