//! No-op implementations for testing and placeholder use.
mod noop_grpc_ingress_impl;
mod noop_grpc_validator_impl;
pub use noop_grpc_ingress_impl::NoopGrpcIngress;
pub use noop_grpc_validator_impl::{NoopGrpcValidator, Validator};
