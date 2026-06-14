//! SAF — config-loading public factory surface.
//!
//! Public entry-point for all `ServerConfigLoader` config-loading operations.
//! The method implementations are defined via `impl ServerConfigLoader` in
//! [`server_svc`](super::super::server_svc).

pub use crate::api::config::{ConfigError, RuntimeConfig};
pub use crate::api::runtime::ServerConfigLoader;
