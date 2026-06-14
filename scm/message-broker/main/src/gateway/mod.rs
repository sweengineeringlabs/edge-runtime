//! Gateway layer — re-exports the public SAF surface.

pub(crate) mod egress;
pub(crate) mod ingress;

#[cfg(feature = "tokio-rt")]
pub use crate::api::broker::types::in_memory_message_broker::InMemoryMessageBroker;
pub use crate::api::broker::MessageBroker;
pub use crate::api::broker::MessageStream;
pub use crate::saf::*;
