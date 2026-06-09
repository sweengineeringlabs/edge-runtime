//! Gateway layer — re-exports the public SAF surface.

pub(crate) mod egress;
pub(crate) mod ingress;

pub use crate::api::config::traits::Validator;
pub use crate::saf::*;
