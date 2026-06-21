//! Contract traits for gRPC ingress.

mod grpc_ingress;
mod validator;
pub use grpc_ingress::GrpcIngress;
pub use validator::Validator;
