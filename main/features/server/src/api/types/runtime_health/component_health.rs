//! `ComponentHealth` — health status of a single runtime subsystem.

/// Health status of a single runtime subsystem.
#[derive(Debug, Clone)]
pub struct ComponentHealth {
    /// Name of the runtime subsystem (e.g. "http", "grpc", "db").
    pub name: String,
    /// `true` when the subsystem is operating normally.
    pub healthy: bool,
    /// Optional human-readable explanation when `healthy` is `false`.
    pub detail: Option<String>,
}

impl ComponentHealth {
    /// Construct a healthy component with no detail message.
    pub fn healthy(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            healthy: true,
            detail: None,
        }
    }
    /// Construct an unhealthy component with a detail message.
    pub fn unhealthy(name: impl Into<String>, detail: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            healthy: false,
            detail: Some(detail.into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: healthy
    #[test]
    fn test_component_healthy_has_no_detail() {
        let c = ComponentHealth::healthy("db");
        assert!(c.healthy && c.detail.is_none());
    }

    /// @covers: unhealthy
    #[test]
    fn test_component_unhealthy_has_detail() {
        let c = ComponentHealth::unhealthy("db", "connection refused");
        assert!(!c.healthy && c.detail.is_some());
    }
}
