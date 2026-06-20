//! Public contract declarations — traits, types, and errors.

mod error;
mod noop;
mod traits;
mod types;

pub use error::GrpcIngressError;
pub use noop::{NoopGrpcIngress, NoopGrpcValidator};
pub use traits::{GrpcIngress, Validator};
pub use types::{GrpcHealthCheck, GrpcIngressResult, GrpcMethod, GrpcRequest, GrpcResponse};
