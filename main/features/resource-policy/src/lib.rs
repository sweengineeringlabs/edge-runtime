//! `swe-edge-runtime-resource-policy` — config-driven resource limits (ADR-005).
//!
//! Provides [`ResourcePolicy`] (named, fully-resolved limits), [`ResourceLimitsResolver`]
//! (pure multi-level derivation chain), and [`ResourcePolicyRunner`] (a
//! [`ProcessRunner`] decorator that injects policy into every [`ProcessArgs`]).
//!
//! # Quick start
//!
//! ```rust,no_run
//! use std::sync::Arc;
//! use swe_edge_configbuilder::{ConfigSection as _, create_loader};
//! use swe_edge_egress_process::{process_runner, ProcessRunner as _};
//! use swe_edge_runtime_resource_policy::{
//!     ResourcePolicyConfig, ResourcePolicyRunner, create_resource_policy_runner,
//! };
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let loader = create_loader();
//! let config = ResourcePolicyConfig::load(&loader)?;
//! let policy = config.get("default")?;
//! let runner = create_resource_policy_runner(Arc::new(process_runner()), policy);
//! # Ok(())
//! # }
//! ```
//!
//! [`ProcessRunner`]: swe_edge_egress_process::ProcessRunner
//! [`ProcessArgs`]: swe_edge_egress_process::ProcessArgs

#![warn(missing_docs)]
#![deny(unsafe_code)]

mod api;
mod core;
mod saf;

pub use saf::*;
