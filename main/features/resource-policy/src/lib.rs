//! `swe-edge-runtime-resource-policy` — config-driven resource limits (ADR-005).
//!
//! Provides [`ResourcePolicy`] (named, fully-resolved limits), [`ResourceLimitsResolver`]
//! (pure multi-level derivation chain), and [`ResourcePolicyRunner`] (a
//! [`SubprocessRunner`] decorator that injects policy into every [`SubprocessArgs`]).
//!
//! # Quick start
//!
//! ```rust,no_run
//! use std::sync::Arc;
//! use swe_edge_configbuilder::{ConfigLoaderFactory, ConfigSection as _};
//! use swe_edge_egress_subprocess::{SubprocessSvc, SubprocessRunner as _};
//! use swe_edge_runtime_resource_policy::{
//!     ResourcePolicyConfig, ResourcePolicyRunner, create_resource_policy_runner,
//! };
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let loader = ConfigLoaderFactory::create_loader()?;
//! let config = ResourcePolicyConfig::load(&loader)?;
//! let policy = config.get("default")?;
//! let runner = create_resource_policy_runner(Arc::new(SubprocessSvc::runner()), policy);
//! # Ok(())
//! # }
//! ```
//!
//! [`SubprocessRunner`]: swe_edge_egress_subprocess::SubprocessRunner
//! [`SubprocessArgs`]: swe_edge_egress_subprocess::SubprocessArgs

#![warn(missing_docs)]
#![deny(unsafe_code)]

mod api;
mod core;
mod saf;

pub use saf::*;
