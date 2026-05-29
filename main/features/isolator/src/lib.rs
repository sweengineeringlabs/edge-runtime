//! `swe-edge-runtime-isolator` — OS-level process isolation profiles (ADR-004).
//!
//! Provides named [`IsolationProfile`] implementations loaded from TOML config.
//! Consumers resolve a profile by name from [`IsolationProfileRegistry`] and
//! attach it to [`SubprocessArgs`] before passing to [`SubprocessRunner`].
//!
//! # Quick start
//!
//! ```rust,no_run
//! use swe_edge_configbuilder::ConfigLoaderFactory;
//! use swe_edge_runtime_isolator::IsolatorSvc;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let loader   = ConfigLoaderFactory::create_loader()?;
//! let registry = IsolatorSvc::create_profile_registry(&loader)?;
//! let profile  = registry.get("default")?;
//! # Ok(())
//! # }
//! ```
//!
//! [`IsolationProfile`]: swe_edge_egress_subprocess::IsolationProfile
//! [`SubprocessArgs`]: swe_edge_egress_subprocess::SubprocessArgs
//! [`SubprocessRunner`]: swe_edge_egress_subprocess::SubprocessRunner

#![warn(missing_docs)]
#![deny(unsafe_code)]

mod api;
mod core;
mod gateway;
mod saf;

pub use gateway::egress::*;
