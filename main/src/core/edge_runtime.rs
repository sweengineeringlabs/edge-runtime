//! `EdgeRuntimeBuilder::serve()` implementation.

use std::sync::Arc;
use std::time::Duration;

use edge_proxy::new_null_lifecycle_monitor;
use swe_edge_ingress::{AxumHttpServer, TonicGrpcServer};
use swe_edge_ingress_verifier::{JwtVerifier, TokenVerifier};
use tokio::sync::oneshot;

use crate::api::edge_runtime::EdgeRuntimeBuilder;
use crate::api::error::{RuntimeError, RuntimeResult};
use crate::api::input::{DefaultInput, Input};
use crate::api::output::DefaultOutput;
use crate::saf::{load_config_xdg, run_until_signal, runtime_manager};

const DEFAULT_APP_NAME: &str = "swe-edge";

impl EdgeRuntimeBuilder {
    /// Assemble all registered components and start the runtime.
    ///
    /// Blocks until SIGTERM / SIGINT or an error.
    pub async fn serve(self) -> RuntimeResult<()> {
        let config = match self.config {
            Some(c) => c,
            None => {
                let name = self.app_name.as_deref().unwrap_or(DEFAULT_APP_NAME);
                load_config_xdg(name).map_err(|e| RuntimeError::StartFailed(e.to_string()))?
            }
        };

        // ── Resolve TLS / auth: builder explicit wins, else fall back to config ─
        let http_tls = self.http_tls.or_else(|| config.http_tls.clone());
        let grpc_tls = self.grpc_tls.or_else(|| config.grpc_tls.clone());

        let http_bearer_verifier: Option<Arc<dyn TokenVerifier>> =
            if let Some(v) = self.http_bearer_verifier {
                Some(v)
            } else if let Some(ref auth_cfg) = config.http_auth {
                Some(Arc::new(
                    JwtVerifier::from_config(auth_cfg)
                        .map_err(|e| RuntimeError::StartFailed(e.to_string()))?,
                ))
            } else {
                None
            };

        let grpc_allow_unauthenticated =
            self.grpc_allow_unauthenticated || config.grpc_allow_unauthenticated;

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
            if let Some(tls)      = http_tls              { server = server.with_tls(tls); }
            if let Some(verifier) = http_bearer_verifier  { server = server.with_bearer_auth(verifier); }
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
            if let Some(tls) = grpc_tls { server = server.with_tls(tls); }
            if !self.grpc_interceptors.is_empty() {
                server = server.with_interceptors(self.grpc_interceptors);
            }
            if grpc_allow_unauthenticated {
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
