//! SAF layer — gRPC runtime-contracts public facade.

mod grpc_ingress_svc;
mod validator_svc;

pub use grpc_ingress_svc::{
    GrpcHealthCheck, GrpcIngress, GrpcIngressError, GrpcIngressResult, GrpcMethod, GrpcRequest,
    GrpcResponse, NoopGrpcIngress,
};
pub use validator_svc::{NoopGrpcValidator, Validator};
