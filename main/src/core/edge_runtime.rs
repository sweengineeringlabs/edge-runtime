//! `EdgeRuntimeBuilder::serve()` implementation.

use std::sync::Arc;
use std::time::Duration;

use edge_proxy::new_null_lifecycle_monitor;
use swe_edge_ingress::{AxumHttpServer, TonicGrpcServer};
use tokio::sync::oneshot;

use crate::api::edge_runtime::EdgeRuntimeBuilder;
use crate::api::error::{RuntimeError, RuntimeResult};
use crate::api::input::{DefaultInput, Input};
use crate::api::output::DefaultOutput;
use crate::saf::{load_config, run_until_signal, runtime_manager};

impl EdgeRuntimeBuilder {
    /// Assemble all registered components and start the runtime.
    ///
    /// Blocks until SIGTERM / SIGINT or an error.
    pub async fn serve(self) -> RuntimeResult<()> {
        let config = match self.config {
            Some(c) => c,
            None    => load_config().map_err(|e| RuntimeError::StartFailed(e.to_string()))?,
        };

        // ── Ingress ───────────────────────────────────────────────────────────
        let mut input = DefaultInput::empty();
        if let Some(d) = self.http_dispatcher { input = input.with_http(Arc::new(d)); }
        else if let Some(h) = self.http_handler { input = input.with_http(h); }

        if let Some(d) = self.grpc_dispatcher { input = input.with_grpc(Arc::new(d)); }
        else if let Some(h) = self.grpc_handler { input = input.with_grpc(h); }

        if !input.has_any() {
            return Err(RuntimeError::StartFailed(
                "EdgeRuntime: no handler registered — call .http_route() or .grpc_route()".into(),
            ));
        }

        // ── Egress ────────────────────────────────────────────────────────────
        let egress_http = self.egress_http.ok_or_else(|| {
            RuntimeError::StartFailed(
                "EdgeRuntime: no HTTP egress client registered — call .egress_http()".into(),
            )
        })?;
        let mut output = DefaultOutput::new_http(egress_http);
        if let Some(g) = self.egress_grpc { output = output.with_grpc(g); }

        let lifecycle = self.lifecycle.unwrap_or_else(|| new_null_lifecycle_monitor());

        // ── Servers ───────────────────────────────────────────────────────────
        let timeout_secs = config.shutdown_timeout_secs;
        let http_bind    = config.http_bind.clone();
        let grpc_bind    = config.grpc_bind.clone();

        let (http_tx, http_rx) = oneshot::channel::<()>();
        let http_task = input.http().map(|handler| {
            let mut server = AxumHttpServer::new(http_bind, handler);
            if let Some(tls)      = self.http_tls              { server = server.with_tls(tls); }
            if let Some(verifier) = self.http_bearer_verifier  { server = server.with_bearer_auth(verifier); }
            tokio::spawn(async move {
                let signal = async move { let _ = http_rx.await; };
                if let Err(e) = server.serve(signal).await {
                    tracing::error!("HTTP server error: {e}");
                }
            })
        });

        let (grpc_tx, grpc_rx) = oneshot::channel::<()>();
        let grpc_task = input.grpc().map(|handler| {
            let mut server = TonicGrpcServer::new(grpc_bind, handler);
            if let Some(tls) = self.grpc_tls { server = server.with_tls(tls); }
            if !self.grpc_interceptors.is_empty() {
                server = server.with_interceptors(self.grpc_interceptors);
            }
            if self.grpc_allow_unauthenticated {
                server = server.allow_unauthenticated(true);
            }
            tokio::spawn(async move {
                let signal = async move { let _ = grpc_rx.await; };
                if let Err(e) = server.serve(signal).await {
                    tracing::error!("gRPC server error: {e}");
                }
            })
        });

        let mgr    = runtime_manager(config, Arc::new(input), Arc::new(output), lifecycle);
        let result = run_until_signal(mgr, timeout_secs, wait_for_signal()).await;

        let _ = http_tx.send(());
        let _ = grpc_tx.send(());
        if let Some(t) = http_task { let _ = tokio::time::timeout(Duration::from_secs(5), t).await; }
        if let Some(t) = grpc_task { let _ = tokio::time::timeout(Duration::from_secs(5), t).await; }

        result
    }
}

async fn wait_for_signal() {
    #[cfg(unix)]
    {
        use tokio::signal::unix::{signal, SignalKind};
        let mut sigterm = match signal(SignalKind::terminate()) {
            Ok(s) => s,
            Err(e) => {
                tracing::warn!("could not register SIGTERM handler: {e}");
                let _ = tokio::signal::ctrl_c().await;
                return;
            }
        };
        tokio::select! {
            _ = sigterm.recv()           => {}
            _ = tokio::signal::ctrl_c() => {}
        }
    }
    #[cfg(not(unix))]
    { let _ = tokio::signal::ctrl_c().await; }
}
