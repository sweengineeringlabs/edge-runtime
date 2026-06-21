//! `CompositeGrpcIngress` — re-exported from the composite types module.

pub use crate::api::composite::types::composite_grpc_ingress::CompositeGrpcIngress;
pub use crate::api::composite::types::composite_grpc_ingress::MAX_GRPC_COMPOSITE_HANDLERS;

/// Maximum number of concurrent protocols a composite ingress may multiplex.
pub const COMPOSITE_PROTOCOL_LIMIT: usize = 2;
