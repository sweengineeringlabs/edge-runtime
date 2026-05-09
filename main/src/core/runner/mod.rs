//! Testable daemon runner — start, await signal, shut down within timeout.

use std::time::Duration;

use crate::api::error::{RuntimeError, RuntimeResult};
use crate::api::runtime_manager::RuntimeManager;

/// Start `manager`, await `signal`, then shut down within `shutdown_timeout_secs`.
///
/// Consumers should use [`crate::saf::daemon::run`] or
/// [`EdgeRuntimeBuilder::serve`](crate::api::edge_runtime::EdgeRuntimeBuilder).
pub(crate) async fn run_until_signal<F>(
    manager:               impl RuntimeManager,
    shutdown_timeout_secs: u64,
    signal:                F,
) -> RuntimeResult<()>
where
    F: std::future::Future<Output = ()>,
{
    manager.start().await?;
    tracing::info!("daemon ready — awaiting shutdown signal");
    signal.await;
    tracing::info!(timeout_secs = shutdown_timeout_secs, "shutdown signal received — draining");
    tokio::time::timeout(
        Duration::from_secs(shutdown_timeout_secs),
        manager.shutdown(),
    )
    .await
    .map_err(|_| RuntimeError::ShutdownTimeout(shutdown_timeout_secs))?
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::future::BoxFuture;
    use crate::api::types::{RuntimeHealth, RuntimeStatus};

    struct OkManager;
    impl RuntimeManager for OkManager {
        fn start(&self)    -> BoxFuture<'_, RuntimeResult<()>> { Box::pin(async { Ok(()) }) }
        fn shutdown(&self) -> BoxFuture<'_, RuntimeResult<()>> { Box::pin(async { Ok(()) }) }
        fn health(&self)   -> BoxFuture<'_, RuntimeHealth> {
            Box::pin(async { RuntimeHealth { status: RuntimeStatus::Running, components: vec![], uptime_secs: 0 } })
        }
    }

    struct FailStartManager;
    impl RuntimeManager for FailStartManager {
        fn start(&self) -> BoxFuture<'_, RuntimeResult<()>> {
            Box::pin(async { Err(RuntimeError::StartFailed("injected".into())) })
        }
        fn shutdown(&self) -> BoxFuture<'_, RuntimeResult<()>> { Box::pin(async { Ok(()) }) }
        fn health(&self) -> BoxFuture<'_, RuntimeHealth> {
            Box::pin(async { RuntimeHealth { status: RuntimeStatus::Stopped, components: vec![], uptime_secs: 0 } })
        }
    }

    struct HangShutdownManager;
    impl RuntimeManager for HangShutdownManager {
        fn start(&self)    -> BoxFuture<'_, RuntimeResult<()>> { Box::pin(async { Ok(()) }) }
        fn shutdown(&self) -> BoxFuture<'_, RuntimeResult<()>> { Box::pin(std::future::pending()) }
        fn health(&self) -> BoxFuture<'_, RuntimeHealth> {
            Box::pin(async { RuntimeHealth { status: RuntimeStatus::Running, components: vec![], uptime_secs: 0 } })
        }
    }

    /// @covers: run_until_signal
    #[tokio::test]
    async fn test_run_until_signal_starts_and_shuts_down_cleanly() {
        let result = run_until_signal(OkManager, 30, std::future::ready(())).await;
        assert!(result.is_ok());
    }

    /// @covers: run_until_signal — propagates start failure
    #[tokio::test]
    async fn test_run_until_signal_propagates_start_failure() {
        let err = run_until_signal(FailStartManager, 30, std::future::ready(())).await.unwrap_err();
        assert!(matches!(err, RuntimeError::StartFailed(_)));
    }

    /// @covers: run_until_signal — shutdown timeout
    #[tokio::test(start_paused = true)]
    async fn test_run_until_signal_returns_shutdown_timeout_when_drain_exceeds_limit() {
        use std::time::Duration;
        let timeout_secs = 5_u64;
        let fut = run_until_signal(HangShutdownManager, timeout_secs, std::future::ready(()));
        tokio::pin!(fut);
        tokio::time::advance(Duration::from_secs(timeout_secs + 1)).await;
        let err = fut.await.unwrap_err();
        assert!(matches!(err, RuntimeError::ShutdownTimeout(5)));
    }
}
