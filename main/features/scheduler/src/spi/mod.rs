//! SPI — Service Provider Interface for runtime implementations.
//!
//! Providers extend and override core/ logic for specific runtimes.
//! Each runtime (tokio, etc.) implements the provider interface here.
//!
//! **Note**: All runtime modules are private. Consumers never see TokioScheduler
//! or any implementation types. SAF exposes only factories that return `impl Trait`,
//! hiding all implementation details.

#[cfg(feature = "tokio-rt")]
mod tokio;

#[cfg(feature = "tokio-rt")]
pub(crate) use tokio::TokioScheduler;
