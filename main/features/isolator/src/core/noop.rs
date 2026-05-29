//! `NoopIsolator` — no-op isolation profile; safe default for dev and CI.

use swe_edge_egress_subprocess::{IsolationError, IsolationProfile};

/// Applies no OS-level restrictions to the subprocess.
///
/// Always succeeds. Never use in production for untrusted or arbitrary code.
#[derive(Debug)]
pub(crate) struct NoopIsolator;

impl IsolationProfile for NoopIsolator {
    fn name(&self) -> &str {
        "noop"
    }

    fn configure(&self, _cmd: &mut tokio::process::Command) -> Result<(), IsolationError> {
        Ok(())
    }

    fn apply(&self, _child: &mut tokio::process::Child) -> Result<(), IsolationError> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_noop_isolator_name_is_noop() {
        assert_eq!(NoopIsolator.name(), "noop");
    }

    #[tokio::test]
    async fn test_noop_isolator_configure_returns_ok() {
        let mut cmd = tokio::process::Command::new("echo");
        assert!(NoopIsolator.configure(&mut cmd).is_ok());
    }
}
