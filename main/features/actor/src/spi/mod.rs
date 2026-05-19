//! SPI — Service Provider Interface layer with runtime implementations.
//!
//! The SPI layer contains implementations of the Actor API for different runtimes.
//! Each implementation (tokio, async-std, etc.) is feature-gated.

#[cfg(feature = "tokio-rt")]
mod tokio_actor_handle;

#[cfg(feature = "tokio-rt")]
mod tokio_mailbox;

#[cfg(feature = "tokio-rt")]
mod tokio_stop_handle;

#[cfg(feature = "tokio-rt")]
pub(crate) use tokio_actor_handle::TokioActorHandle;

#[cfg(feature = "tokio-rt")]
pub(crate) use tokio_mailbox::{spawn_tokio_actor, spawn_tokio_actor_with_stop};

#[cfg(feature = "tokio-rt")]
pub(crate) use tokio_stop_handle::TokioStopHandle;
