//! Edge runtime entry-point and builder.

pub(crate) mod edge;
pub(crate) mod runtime_builder_serve;

pub use edge::Runtime;
pub use edge::RuntimeBuilder;
