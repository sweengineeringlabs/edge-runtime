//! HTTP ingress/egress contracts for the swe-edge runtime layer.
//!
//! This crate is **contracts only** — no Axum, no TLS, no connection-pool dependencies.
//!
//! # Consumers
//! - Plugins implement [`saf::HttpIngress`] to expose an HTTP surface.
//! - Transport crates implement [`saf::HttpIngress`] for their server.
//! - The composition root wires both sides together.
//! - [`saf::NoopHttpIngress`] is provided as a test helper.
#![deny(unsafe_code)]
#![warn(missing_docs)]

mod api;
mod core;
mod saf;
mod spi;

pub use saf::*;
