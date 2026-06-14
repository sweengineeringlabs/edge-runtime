//! SAF — `CompositeIngress` public service surface.
pub use crate::api::composite::traits::composite_ingress::CompositeIngress;
pub use crate::api::composite::types::composite_grpc_ingress::CompositeGrpcIngress;
/// Identifies the `CompositeIngress` SAF contract in this crate.
pub const COMPOSITE_INGRESS_SVC: &str = "composite_ingress";
