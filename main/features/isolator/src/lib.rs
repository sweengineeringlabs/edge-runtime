//! `swe-edge-runtime-isolator` — OS-level process isolation profiles (ADR-004).
//!
//! Provides named [`IsolationProfile`] implementations loaded from TOML config.
//! Consumers resolve a profile by name from [`IsolationProfileRegistry`] and
//! attach it to [`ProcessArgs`] before passing to [`ProcessRunner`].
//!
//! # Quick start
//!
//! ```rust,no_run
//! use swe_edge_configbuilder::{ConfigSection as _, create_loader};
//! use swe_edge_runtime_isolator::{IsolatorConfig, IsolationProfileRegistry};
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let loader   = create_loader();
//! let config   = IsolatorConfig::load(&loader)?;
//! let registry = IsolationProfileRegistry::from_config(config)?;
//! let profile  = registry.get("default")?;
//! # Ok(())
//! # }
//! ```
//!
//! [`IsolationProfile`]: swe_edge_egress_subprocess::IsolationProfile
//! [`ProcessArgs`]: swe_edge_egress_subprocess::ProcessArgs
//! [`ProcessRunner`]: swe_edge_egress_subprocess::ProcessRunner

#![warn(missing_docs)]
#![deny(unsafe_code)]

mod api;
mod core;
mod saf;

pub use saf::*;
