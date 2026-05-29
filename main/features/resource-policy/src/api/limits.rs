//! `ResourceLimits` — optional resource dimensions used in the resolver chain.

/// Optional resource dimensions for one level of the resolver chain.
///
/// All fields are `Option<u64>` — `None` means "defer to the next layer".
/// A resolved `Some(0)` means "unlimited" for CPU time and memory.
/// Construct this from step YAML, agent YAML, or operator TOML and pass
/// layers to [`ResourceLimitsResolver`].
///
/// [`ResourceLimitsResolver`]: crate::ResourceLimitsResolver
#[derive(Debug, Clone, Default, serde::Deserialize, serde::Serialize)]
pub struct ResourceLimits {
    /// Wall-clock timeout in milliseconds. `None` → defer to next layer.
    pub timeout_ms: Option<u64>,

    /// Maximum combined stdout + stderr bytes. `None` → defer to next layer.
    pub output_bytes_cap: Option<u64>,

    /// CPU time limit in milliseconds (user + system). `None` → defer.
    /// `Some(0)` = unlimited once resolved.
    pub cpu_time_ms: Option<u64>,

    /// Maximum virtual address space in bytes. `None` → defer.
    /// `Some(0)` = unlimited once resolved.
    pub memory_bytes: Option<u64>,
}

impl ResourceLimits {
    /// Returns `true` if all fields are `None` (this layer contributes nothing).
    pub fn is_empty(&self) -> bool {
        self.timeout_ms.is_none()
            && self.output_bytes_cap.is_none()
            && self.cpu_time_ms.is_none()
            && self.memory_bytes.is_none()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: is_empty
    #[test]
    fn test_resource_limits_default_is_empty() {
        assert!(ResourceLimits::default().is_empty());
    }

    /// @covers: is_empty
    #[test]
    fn test_resource_limits_with_any_field_is_not_empty() {
        let l = ResourceLimits {
            timeout_ms: Some(1_000),
            ..Default::default()
        };
        assert!(!l.is_empty());
    }

    /// @covers: is_empty
    #[test]
    fn test_resource_limits_zero_cpu_is_not_empty() {
        let l = ResourceLimits {
            cpu_time_ms: Some(0),
            ..Default::default()
        };
        assert!(!l.is_empty());
    }
}
