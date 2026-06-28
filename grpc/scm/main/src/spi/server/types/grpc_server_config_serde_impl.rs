//! Serde deserialization helpers for GrpcServerConfig.

use crate::api::{
    GrpcServerConfig, DEFAULT_KEEPALIVE_INTERVAL_SECS, DEFAULT_KEEPALIVE_TIMEOUT_SECS,
};

impl GrpcServerConfig {
    /// Default keepalive interval returned by serde when the field is absent.
    #[allow(non_snake_case)]
    pub(crate) fn default_keepalive_interval_secs() -> Option<u64> {
        Some(DEFAULT_KEEPALIVE_INTERVAL_SECS)
    }

    /// Default keepalive timeout returned by serde when the field is absent.
    #[allow(non_snake_case)]
    pub(crate) fn default_keepalive_timeout_secs() -> u64 {
        DEFAULT_KEEPALIVE_TIMEOUT_SECS
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_keepalive_interval_secs_returns_some() {
        assert_eq!(
            GrpcServerConfig::default_keepalive_interval_secs(),
            Some(DEFAULT_KEEPALIVE_INTERVAL_SECS)
        );
    }

    #[test]
    fn test_default_keepalive_timeout_secs_returns_default() {
        assert_eq!(
            GrpcServerConfig::default_keepalive_timeout_secs(),
            DEFAULT_KEEPALIVE_TIMEOUT_SECS
        );
    }
}
