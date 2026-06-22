//! HTTP server binding for the swe-edge runtime layer.
//!
//! Provides the Axum-backed server implementation over the
//! `swe-edge-ingress-http` port contracts.
//!
//! # Consumers
//! - The composition root wires an [`HttpIngress`] implementor into [`AxumHttpServer`].
//! - [`NoopHttpIngress`] is provided as a test helper.
//! - [`HttpServer`] is the trait that drives the server lifecycle.
#![deny(unsafe_code)]
#![warn(missing_docs)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

mod api;
mod core;
mod saf;
mod spi;

pub use api::*;
pub use saf::*;
