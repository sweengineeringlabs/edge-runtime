//! `swe-edge-runtime-grpc` — gRPC server hosting crate for the swe-edge runtime layer.
//!
//! Provides [`TonicGrpcServer`], [`GrpcServerConfig`], the full dispatch loop,
//! and re-exports all ingress-grpc port contract types.

#![warn(missing_docs)]
#![deny(unsafe_code)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]
mod api;
mod core;
mod saf;
mod spi;

pub use saf::*;
