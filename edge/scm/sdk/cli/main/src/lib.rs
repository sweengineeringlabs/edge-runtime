//! CLI runner/command contracts for the swe-edge runtime layer.
//!
//! This crate is **contracts only** — no clap, no argh, no structopt dependencies.
//!
//! # Consumers
//! - Plugins implement [`saf::CliRunner`] to expose a CLI surface.
//! - The composition root wires a concrete runner into the CLI entrypoint.
//! - [`saf::NoopCliRunner`] is provided as a test helper.
#![deny(unsafe_code)]
#![warn(missing_docs)]

mod api;
mod core;
mod saf;
mod spi;

pub use saf::*;
