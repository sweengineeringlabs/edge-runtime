//! SAF — daemon runner: assemble, start, and drain.

use std::sync::Arc;
use std::time::Duration;

use crate::api::error::RuntimeResult;
use crate::api::input::Input;
use crate::api::output::Output;
use crate::api::runtime_manager::RuntimeManager;
use crate::api::types::RuntimeConfig;
use crate::core::runner::run_until_signal;
use crate::core::DefaultRuntimeManager;
use edge_proxy::LifecycleMonitor;

/// Assemble a [`RuntimeManager`] from the supplied config, ingress, egress,
/// and lifecycle monitor.
pub fn runtime_manager(
    config:    RuntimeConfig,
    ingress:   Arc<dyn Input>,
    egress:    Arc<dyn Output>,
    lifecycle: Arc<dyn LifecycleMonitor>,
) -> impl RuntimeManager {
    DefaultRuntimeManager::new(config, ingress, egress, lifecycle)
}

/// Start the daemon and block until a shutdown signal is received.
///
/// Calls [`RuntimeManager::start`], waits for `SIGTERM` or `SIGINT`
/// (Ctrl+C on all platforms), then calls [`RuntimeManager::shutdown`].
/// If shutdown does not complete within `config.shutdown_timeout_secs`,
/// returns [`RuntimeError::ShutdownTimeout`].
///
/// If `ingress` includes an HTTP transport, an Axum HTTP server is
/// spawned automatically against `config.http_bind`. If `ingress` includes
/// a gRPC transport, a gRPC server is spawned against `config.grpc_bind`.
/// Both servers are signalled to drain before `RuntimeManager::shutdown`.
pub async fn run(
    config:    RuntimeConfig,
    ingress:   Arc<dyn Input>,
    egress:    Arc<dyn Output>,
    lifecycle: Arc<dyn LifecycleMonitor>,
) -> RuntimeResult<()> {
    use swe_edge_ingress::{AxumHttpServer, TonicGrpcServer};
    use tokio::sync::oneshot;

    let timeout_secs = config.shutdown_timeout_secs;
    let http_bind    = config.http_bind.clone();
    let grpc_bind    = config.grpc_bind.clone();

    let (http_shutdown_tx, http_shutdown_rx) = oneshot::channel::<()>();
    let http_task = ingress.http().map(|handler| {
        let server = AxumHttpServer::new(http_bind, handler);
        tokio::spawn(async move {
            let signal = async move { let _ = http_shutdown_rx.await; };
            if let Err(e) = server.serve(signal).await {
                tracing::error!("HTTP server error: {e}");
            }
        })
    });

    let (grpc_shutdown_tx, grpc_shutdown_rx) = oneshot::channel::<()>();
    let grpc_task = ingress.grpc().map(|handler| {
        let server = TonicGrpcServer::new(grpc_bind, handler);
        tokio::spawn(async move {
            let signal = async move { let _ = grpc_shutdown_rx.await; };
            if let Err(e) = server.serve(signal).await {
                tracing::error!("gRPC server error: {e}");
            }
        })
    });

    let mgr    = runtime_manager(config, ingress, egress, lifecycle);
    let result = run_until_signal(mgr, timeout_secs, wait_for_signal()).await;

    let _ = http_shutdown_tx.send(());
    let _ = grpc_shutdown_tx.send(());
    if let Some(task) = http_task {
        let _ = tokio::time::timeout(Duration::from_secs(5), task).await;
    }
    if let Some(task) = grpc_task {
        let _ = tokio::time::timeout(Duration::from_secs(5), task).await;
    }

    result
}

/// Wait for SIGTERM or SIGINT, whichever arrives first.
async fn wait_for_signal() {
    #[cfg(unix)]
    {
        use tokio::signal::unix::{signal, SignalKind};
        let mut sigterm = match signal(SignalKind::terminate()) {
            Ok(s) => s,
            Err(e) => {
                tracing::warn!("could not register SIGTERM handler: {e} — falling back to SIGINT only");
                let _ = tokio::signal::ctrl_c().await;
                return;
            }
        };
        tokio::select! {
            _ = sigterm.recv()           => tracing::info!("SIGTERM received"),
            _ = tokio::signal::ctrl_c() => tracing::info!("SIGINT received"),
        }
    }
    #[cfg(not(unix))]
    {
        let _ = tokio::signal::ctrl_c().await;
        tracing::info!("SIGINT received");
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    /// @covers: runtime_manager
    #[test]
    fn test_runtime_manager_constructs_without_panic() {
        use swe_edge_ingress::HttpInbound;
        use edge_proxy::new_null_lifecycle_monitor;
        use crate::api::types::RuntimeConfig;
        use crate::api::input::DefaultInput;
        use crate::api::output::DefaultOutput;
        use swe_edge_egress_http::default_http_outbound;

        let http = Arc::new(
            default_http_outbound().expect("default http outbound"),
        );
        let input  = Arc::new(DefaultInput::empty());
        let output = Arc::new(DefaultOutput::new_http(http));
        let lc     = new_null_lifecycle_monitor();
        let _mgr   = runtime_manager(RuntimeConfig::default(), input, output, lc);
    }

    /// @covers: run
    #[tokio::test]
    async fn test_run_fails_when_no_ingress_configured() {
        use edge_proxy::new_null_lifecycle_monitor;
        use crate::api::{input::DefaultInput, output::DefaultOutput, error::RuntimeError};
        use swe_edge_egress_http::default_http_outbound;

        let http   = Arc::new(default_http_outbound().expect("http outbound"));
        let input  = Arc::new(DefaultInput::empty());
        let output = Arc::new(DefaultOutput::new_http(http));
        let lc     = new_null_lifecycle_monitor();
        let err    = run(RuntimeConfig::default(), input, output, lc).await.unwrap_err();
        assert!(matches!(err, RuntimeError::StartFailed(_)),
            "expected StartFailed for empty ingress, got: {err:?}");
    }
}
