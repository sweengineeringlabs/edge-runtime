//! Core implementations — resolver and policy runner.

pub(crate) mod policy_runner;
pub(crate) mod resolver;
pub(crate) mod swe;

pub(crate) use swe::DefaultSweEdgeRuntimeResourcePolicy;
