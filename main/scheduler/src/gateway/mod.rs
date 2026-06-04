//! Gateway layer — I/O boundary adapters for the scheduler.

pub(crate) mod egress;
pub(crate) mod ingress;

pub use crate::saf::*;
