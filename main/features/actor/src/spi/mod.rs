//! SPI — Service Provider Interface for runtime implementations.
//!
//! Providers extend and override core/ logic for specific runtimes.
//! Each runtime (tokio, async-std, etc.) implements the provider interface here.

#[cfg(feature = "tokio-rt")]
pub mod tokio;

#[cfg(feature = "tokio-rt")]
pub(crate) use tokio::{spawn_tokio_actor, spawn_tokio_actor_with_stop};
