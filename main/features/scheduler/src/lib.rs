//! `swe_edge_runtime_scheduler` — async executor for standalone binaries.
//!
//! Consumers who embed `swe-edge-runtime` inside an existing tokio application
//! call [`RuntimeBuilder::serve`] directly from their own async context.
//!
//! Consumers who need a standalone binary with no tokio boilerplate depend on
//! this crate and call [`run`] or [`RuntimeBuilderExt::run`] instead.
//!
//! Single entry point: [`crate::saf`] (edge_runtime_scheduler_svc).

mod api;
mod core;
mod saf;
mod spi;

pub use crate::saf::*;
