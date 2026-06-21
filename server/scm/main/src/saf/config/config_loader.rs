//! SAF — config-loading public factory surface.
//!
//! Public entry-point for all `ServerConfigLoader` config-loading operations.
//! The method implementations are defined via `impl ServerConfigLoader` in
//! `server_svc`.

pub use crate::api::ServerConfigLoader;
pub use crate::api::{ConfigError, RuntimeConfig};
