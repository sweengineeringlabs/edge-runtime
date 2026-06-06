//! Broker SPI — external-library message broker backends.

#[cfg(feature = "kafka")]
pub(crate) mod kafka;
#[cfg(feature = "nats")]
pub(crate) mod nats;
