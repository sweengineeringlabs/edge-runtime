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
//! use swe_edge_configbuilder::ConfigLoaderFactory;
//! use swe_edge_egress_subprocess::{SubprocessSvc, SubprocessRunner as _};
//! use swe_edge_runtime_resource_policy::{PolicySvc, ResourcePolicyRunner};
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let loader = ConfigLoaderFactory::create_loader()?;
//! let policy = PolicySvc::load_policy(&loader, "default")?;
//! let runner = PolicySvc::create_policy_runner(Arc::new(SubprocessSvc::runner()), policy);
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
mod gateway;
mod saf;

pub use gateway::egress::*;
