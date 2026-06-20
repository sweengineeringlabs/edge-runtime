//! `swe-edge-runtime-grpc` — gRPC ingress contract crate.
//!
//! Provides [`GrpcIngress`](crate::GrpcIngress) trait and supporting value types.
//! Contains no transport dependencies (no Tonic, no TLS, no connection-pool).
//! Plugins and transport crates implement [`GrpcIngress`](crate::GrpcIngress);
//! the composition root wires them in.

#![deny(unsafe_code)]
#![warn(missing_docs)]

mod api;
mod core;
mod saf;
mod spi;

pub use saf::*;
