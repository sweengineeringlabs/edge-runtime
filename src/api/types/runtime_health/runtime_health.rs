//! `RuntimeHealth` — aggregate health of the running daemon.

use crate::api::types::runtime_health::component_health::ComponentHealth;
use crate::api::types::RuntimeStatus;

/// Aggregate health report for the runtime.
#[derive(Debug, Clone)]
pub struct RuntimeHealth {
    /// Overall runtime status (Starting, Running, Stopping, Stopped).
    pub status:     RuntimeStatus,
    /// Per-subsystem health snapshots.
    pub components: Vec<ComponentHealth>,
    /// Seconds elapsed since the runtime started.
    pub uptime_secs: u64,
}

impl RuntimeHealth {
    /// Returns `true` when status is `Running` and every component is healthy.
    pub fn is_healthy(&self) -> bool {
        self.status.is_healthy() && self.components.iter().all(|c| c.healthy)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: is_healthy
    #[test]
    fn test_runtime_health_is_healthy_when_all_components_up() {
        let h = RuntimeHealth {
            status: RuntimeStatus::Running,
            components: vec![ComponentHealth::healthy("http")],
            uptime_secs: 10,
        };
        assert!(h.is_healthy());
    }

    /// @covers: is_healthy
    #[test]
    fn test_runtime_health_is_unhealthy_if_component_down() {
        let h = RuntimeHealth {
            status: RuntimeStatus::Running,
            components: vec![ComponentHealth::unhealthy("http", "timeout")],
            uptime_secs: 10,
        };
        assert!(!h.is_healthy());
    }
}
