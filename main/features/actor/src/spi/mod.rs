//! SPI — Service Provider Interface layer with runtime implementations.
//!
//! The SPI layer contains implementations of the Actor API for different runtimes.
//! Each implementation (tokio, async-std, etc.) is feature-gated.

#[cfg(feature = "tokio-rt")]
mod tokio;

#[cfg(feature = "tokio-rt")]
pub(crate) use tokio::{spawn_tokio_actor, spawn_tokio_actor_with_stop};
