//! Ingress adapter contract.

pub(crate) mod default_ingress;
#[allow(clippy::module_inception)]
pub(crate) mod ingress;

pub use default_ingress::DefaultIngress;
pub use ingress::Ingress;
