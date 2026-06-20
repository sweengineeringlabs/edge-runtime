//! SAF — `CompositeIngress` public service surface.
pub use crate::api::CompositeGrpcIngress;
pub use crate::api::CompositeIngress;
/// Identifies the `CompositeIngress` SAF contract in this crate.
pub const COMPOSITE_INGRESS_SVC: &str = "composite_ingress";
