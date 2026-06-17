//! SAF — daemon factory surface.
//!
//! Public entry-point for `Runtime::runtime_manager`, `Runtime::run`, and
//! related assembly methods that drive the daemon lifecycle.
//! The method implementations are defined via `impl Runtime` in
//! `server_svc`.

pub use crate::api::runtime::{Runtime, RuntimeConfig, RuntimeError, RuntimeResult};
