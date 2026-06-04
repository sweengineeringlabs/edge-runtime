//! `IsolatorConfig` — TOML-loaded subprocess isolation policy.

use std::collections::HashMap;

use swe_edge_configbuilder::ConfigSection;

use super::profile_spec::ProfileSpec;

/// Subprocess isolation policy loaded from the `[subprocess_policy]` TOML section.
///
/// # TOML example
///
/// ```toml
/// [subprocess_policy.profiles.default]
/// kind = "noop"
///
/// [subprocess_policy.profiles.restricted]
/// kind             = "seccomp"
/// allowed_syscalls = ["read", "write", "exit", "exit_group"]
/// ```
#[derive(Debug, Clone, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct IsolatorConfig {
    /// Named isolation profiles.  The `"noop"` profile is always available
    /// even when absent from config — see [`IsolatorConfig::default`].
    #[serde(default)]
    pub profiles: HashMap<String, ProfileSpec>,
}

impl Default for IsolatorConfig {
    fn default() -> Self {
        let mut profiles = HashMap::new();
        profiles.insert(
            "default".to_owned(),
            ProfileSpec {
                kind: "noop".to_owned(),
                allowed_syscalls: Vec::new(),
                cpu_rate_hundredths: 0,
                memory_limit_bytes: 0,
                kill_on_job_close: true,
            },
        );
        Self { profiles }
    }
}

/// TOML section key for the subprocess isolation policy.
pub const SECTION_NAME: &str = "subprocess_policy";

impl ConfigSection for IsolatorConfig {
    fn section_name() -> &'static str {
        SECTION_NAME
    }
}
