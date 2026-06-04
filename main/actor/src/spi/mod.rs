//! SPI — Service Provider Interface for runtime implementations.
//!
//! Providers extend and override core/ logic for specific runtimes.
//! Each runtime (tokio, async-std, etc.) implements the provider interface here.
//!
//! **Note**: All runtime modules are private. Consumers never see TokioActorHandle,
//! AsyncStdActorHandle, or any implementation types. SAF exposes only factories
//! that return `impl Trait`, hiding all implementation details.

#[cfg(feature = "tokio-rt")]
pub(crate) mod tokio;

#[cfg(feature = "async-std-rt")]
#[path = "async/mod.rs"]
pub(crate) mod r#async;
