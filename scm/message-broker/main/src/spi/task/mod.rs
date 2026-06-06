//! Task SPI — external-library task queue backends.

#[cfg(feature = "kafka")]
pub(crate) mod kafka;
#[cfg(feature = "nats")]
pub(crate) mod nats;
