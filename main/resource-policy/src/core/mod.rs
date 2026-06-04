//! Core implementations — resolver and policy runner.

pub(crate) mod policy;
pub(crate) mod resolver;
pub(crate) mod swe;

#[expect(
    unused_imports,
    reason = "SEA core/ anchor — used when SAF factory is integrated"
)]
pub(crate) use swe::DefaultSweEdgeRuntimeResourcePolicy;
