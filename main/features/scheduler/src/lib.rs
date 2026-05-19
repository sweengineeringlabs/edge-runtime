//! `swe_edge_runtime_scheduler` — async executor for standalone binaries.
//!
//! Consumers who embed `swe-edge-runtime` inside an existing tokio application
//! call [`RuntimeBuilder::serve`] directly from their own async context.
//!
//! Consumers who need a standalone binary with no tokio boilerplate depend on
//! this crate and call [`run`] or [`RuntimeBuilderExt::run`] instead.

mod api;
mod core;
mod gateway;
mod saf;

pub use gateway::*;
