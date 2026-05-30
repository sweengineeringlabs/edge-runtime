//! Core implementations — resolver and policy runner.

pub(crate) mod policy;
pub(crate) mod resolver;
pub(crate) mod swe;

pub(crate) use swe::DefaultSweEdgeRuntimeResourcePolicy;
