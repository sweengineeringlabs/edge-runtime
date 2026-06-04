//! async-std runtime implementation of the actor mailbox.

mod actor_handle;
mod mailbox;
mod stop_handle;

pub(crate) use mailbox::AsyncStdMailbox;
