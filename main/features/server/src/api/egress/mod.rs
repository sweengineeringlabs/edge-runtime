//! Egress adapter contract.

pub(crate) mod default_egress;
#[allow(clippy::module_inception)]
pub(crate) mod egress;

pub use default_egress::DefaultEgress;
pub use egress::Egress;
