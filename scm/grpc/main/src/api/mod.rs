//! Public contract declarations — traits, types, and errors.

mod error;
mod traits;
mod types;

pub use error::GrpcIngressError;
pub use traits::{GrpcIngress, Validator};
pub use types::{
    GrpcHealthCheck, GrpcIngressResult, GrpcMethod, GrpcRequest, GrpcResponse, NoopGrpcIngress,
    NoopGrpcValidator,
};
