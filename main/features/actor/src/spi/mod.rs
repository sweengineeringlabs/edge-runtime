//! SPI — Service Provider Interface for runtime implementations.
//!
//! Providers extend and override core/ logic for specific runtimes.
//! Each runtime (tokio, async-std, etc.) implements the provider interface here.
//!
//! **Note**: All runtime modules are private. Consumers never see TokioActorHandle,
//! AsyncStdActorHandle, or any implementation types. SAF exposes only factories
//! that return `impl Trait`, hiding all implementation details.

#[cfg(feature = "tokio-rt")]
mod tokio;

#[cfg(feature = "async-std-rt")]
mod async_std;

#[cfg(feature = "tokio-rt")]
pub(crate) use tokio::{spawn_tokio_actor, spawn_tokio_actor_with_stop};

#[cfg(feature = "async-std-rt")]
pub(crate) use async_std::{spawn_async_std_actor, spawn_async_std_actor_with_stop};
