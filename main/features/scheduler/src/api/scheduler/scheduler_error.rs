//! [`SchedulerError`] — errors emitted by the scheduler layer.

/// Errors that can occur when the scheduler fails to start or drive a future.
#[derive(Debug, thiserror::Error)]
pub enum SchedulerError {
    /// The async runtime (e.g. tokio) failed to initialise.
    #[error("scheduler failed to start runtime: {0}")]
    StartFailed(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scheduler_error_display_includes_message() {
        let e = SchedulerError::StartFailed("no threads".into());
        assert!(e.to_string().contains("no threads"));
    }
}
