//! No-op stub types for gRPC ingress contracts — exempt from `api_no_orphan_types`.

mod noop_grpc_ingress;
mod noop_grpc_validator;

pub use noop_grpc_ingress::NoopGrpcIngress;
pub use noop_grpc_validator::NoopGrpcValidator;
