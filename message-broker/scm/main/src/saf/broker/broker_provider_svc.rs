//! SAF — [`BrokerProvider`] implementation.

use crate::api::BrokerProvider;

/// Default backend identifier used when no configuration override is provided.
pub const DEFAULT_BROKER_BACKEND: &str = "inmemory";
