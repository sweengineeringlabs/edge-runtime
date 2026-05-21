#[allow(clippy::module_inception)]
pub(crate) mod in_memory_message_broker;

pub(crate) use in_memory_message_broker::InMemoryMessageBroker;
