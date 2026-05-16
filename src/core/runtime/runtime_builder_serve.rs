//! `RuntimeBuilder::serve()` implementation.

/// Primary type for this module (matches filename for Rule 89).
pub(crate) struct RuntimeBuilderServe;

use std::sync::Arc;
use std::time::Duration;

use edge_proxy::new_null_lifecycle_monitor;
use swe_edge_ingress::{AxumHttpServer, TonicGrpcServer};
use swe_edge_egress_grpc::create_transport_from_config;
use swe_edge_egress_http::{default_http_outbound, default_http_outbound_with_config};
use swe_edge_ingress_grpc_reflection::ReflectionService;
use swe_edge_ingress_verifier::{JwtVerifier, TokenVerifier};
use tokio::sync::oneshot;

use crate::api::runtime::RuntimeBuilder;
use crate::api::error::{RuntimeError, RuntimeResult};
use crate::api::ingress::{DefaultIngress, Ingress};
use swe_observ_metrics::create_local_metrics_backend;
use crate::api::config_loader::ConfigLoader;
use crate::api::monitor::{TrafficCounters, SharedCounters};
use crate::api::egress::DefaultEgress;
use crate::core::config_loader::DefaultConfigLoader;
use crate::core::monitor::{BackgroundSampler, GrpcLoadMonitor, HttpLoadMonitor};
use crate::core::metrics_handler::MetricsHandler;
use crate::core::runner::run_until_signal;
use crate::core::runtime_manager::DefaultRuntimeManager;

const DEFAULT_APP_NAME: &str = "swe-edge";

impl RuntimeBuilder {
    /// Assemble all registered components and start the runtime.
    ///
    /// Blocks until SIGTERM / SIGINT or an error.
    pub async fn serve(self) -> RuntimeResult<()> {
        let config = match self.config {
            Some(c) => c,
            None => {
                let name = self.app_name.as_deref().unwrap_or(DEFAULT_APP_NAME);
                DefaultConfigLoader::xdg(name).load()
                    .map_err(|e| RuntimeError::StartFailed(e.to_string()))?
            }
        };

        // Builder explicit wins; fall back to [observability.tracing] from TOML.
        #[cfg(feature = "observability")]
        {
            let tracing_cfg = self.tracing_config.as_ref()
                .or_else(|| config.observability.as_ref().map(|o| &o.tracing));
            if let Some(cfg) = tracing_cfg {
                crate::api::observability::init_tracing(cfg);
            }
        }

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
        // Capture gRPC registry before the dispatcher is consumed into input.
        let reflection_registry = if config.grpc_reflection {
            self.grpc_dispatcher.as_ref().map(|d| Arc::clone(d.registry()))
        } else {
            None
        };

        let mut input = DefaultIngress::empty();
        if let Some(d) = self.http_dispatcher { input = input.with_http(Arc::new(d)); }
        else if let Some(h) = self.http_handler { input = input.with_http(h); }

        if let Some(d) = self.grpc_dispatcher { input = input.with_grpc(Arc::new(d)); }
        else if let Some(h) = self.grpc_handler { input = input.with_grpc(h); }

        if !input.has_any() {
            return Err(RuntimeError::StartFailed(
                "Runtime: no handler registered — call .http_route() or .grpc_route()".into(),
            ));
        }

        // ── Egress: builder explicit > TOML config > default ─────────────────
        let egress_http: Arc<dyn swe_edge_egress_http::HttpOutbound> =
            if let Some(h) = self.egress_http {
                h
            } else if let Some(http_cfg) = config.egress_http.clone() {
                Arc::new(
                    default_http_outbound_with_config(http_cfg)
                        .map_err(|e| RuntimeError::StartFailed(e.to_string()))?,
                )
            } else {
                Arc::new(
                    default_http_outbound()
                        .map_err(|e| RuntimeError::StartFailed(e.to_string()))?,
                )
            };

        let egress_grpc: Option<Arc<dyn swe_edge_egress_grpc::GrpcOutbound>> =
            if let Some(g) = self.egress_grpc {
                Some(g)
            } else if let Some(ref grpc_cfg) = config.egress_grpc {
                Some(
                    create_transport_from_config(grpc_cfg)
                        .map_err(|e| RuntimeError::StartFailed(e.to_string()))?,
                )
            } else {
                None
            };

        let mut output = DefaultEgress::new_http(egress_http);
        if let Some(g) = egress_grpc { output = output.with_grpc(g); }

        let lifecycle = self.lifecycle.unwrap_or_else(|| new_null_lifecycle_monitor());

        // ── Load monitor — shared counters + background sampler ───────────────
        let counters: Option<SharedCounters> = config.metrics.as_ref().map(|_| {
            let c = Arc::new(TrafficCounters::new(Arc::new(create_local_metrics_backend())));
            let sampler = BackgroundSampler::new(Arc::clone(&c), config.autoscale.clone());
            tokio::spawn(async move { sampler.run().await });
            c
        });

        // ── Servers ───────────────────────────────────────────────────────────
        let timeout_secs  = config.shutdown_timeout_secs;
        let http_bind     = config.http_bind.clone();
        let grpc_bind     = config.grpc_bind.clone();
        let metrics_bind  = config.metrics.as_ref().map(|m| m.bind.clone());
        let metrics_path  = config.metrics.as_ref().map(|m| m.path.clone())
            .unwrap_or_else(|| "/metrics".into());

        let (http_tx, http_rx) = oneshot::channel::<()>();
        let http_task = input.http().map(|handler| {
            // Wrap with load monitor if metrics are enabled.
            let handler: Arc<dyn swe_edge_ingress::HttpInbound> =
                if let Some(ref c) = counters {
                    Arc::new(HttpLoadMonitor::new(handler, Arc::clone(c)))
                } else {
                    handler
                };
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
            // Wrap with load monitor if metrics are enabled.
            let handler: Arc<dyn swe_edge_ingress::GrpcInbound> =
                if let Some(ref c) = counters {
                    Arc::new(GrpcLoadMonitor::new(handler, Arc::clone(c)))
                } else {
                    handler
                };
            // Wrap with reflection if enabled and a dispatcher registry was captured.
            let handler: Arc<dyn swe_edge_ingress::GrpcInbound> =
                if let Some(registry) = reflection_registry {
                    Arc::new(crate::core::composite::CompositeGrpcInbound::new(
                        handler,
                        Arc::new(ReflectionService::new(registry)),
                    ))
                } else {
                    handler
                };
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

        // ── Metrics server ────────────────────────────────────────────────────
        let (metrics_tx, metrics_task) = if let (Some(bind), Some(ref c)) = (metrics_bind, &counters) {
            let (tx, rx) = oneshot::channel::<()>();
            let server   = AxumHttpServer::new(bind, Arc::new(MetricsHandler::new(Arc::clone(c), &metrics_path)));
            let task     = tokio::spawn(async move {
                let signal = async move { let _ = rx.await; };
                if let Err(e) = server.serve(signal).await {
                    tracing::error!("metrics server error: {e}");
                }
            });
            (Some(tx), Some(task))
        } else {
            (None, None)
        };

        let mgr    = DefaultRuntimeManager::new(config, Arc::new(input), Arc::new(output), lifecycle);
        let result = run_until_signal(mgr, timeout_secs, wait_for_signal()).await;

        let _ = http_tx.send(());
        let _ = grpc_tx.send(());
        if let Some(tx) = metrics_tx { let _ = tx.send(()); }
        if let Some(t) = http_task    { let _ = tokio::time::timeout(Duration::from_secs(5), t).await; }
        if let Some(t) = grpc_task    { let _ = tokio::time::timeout(Duration::from_secs(5), t).await; }
        if let Some(t) = metrics_task { let _ = tokio::time::timeout(Duration::from_secs(5), t).await; }

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

#[cfg(test)]
mod tests {
    use crate::api::runtime::Runtime;
    use crate::api::error::RuntimeError;

    /// @covers: serve
    #[test]
    fn test_serve_returns_start_failed_when_no_handler_registered() {
        let rt     = tokio::runtime::Runtime::new().unwrap();
        let result = rt.block_on(Runtime::builder().grpc_allow_unauthenticated().serve());
        assert!(
            matches!(result, Err(RuntimeError::StartFailed(_))),
            "expected StartFailed, got: {result:?}",
        );
    }
}
