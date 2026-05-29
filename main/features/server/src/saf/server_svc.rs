//! SAF — runtime server public factory surface.
//!
//! All public functions are methods on factory types (`ServerConfigLoader`,
//! `ServerMonitor`, `Runtime`) to satisfy SEA Rule 191 (no free-standing fns).

use std::sync::Arc;
use std::time::Duration;

use crate::api::config::ConfigError;
use crate::api::config_loader::ConfigLoader;
use crate::api::egress::Egress;
use crate::api::error::{RuntimeError, RuntimeResult};
use crate::api::ingress::Ingress;
use crate::api::runtime::Runtime;
use crate::api::traits::Validator;
use crate::api::types::RuntimeConfig;
use crate::api::types::ServerConfigLoader;
use crate::api::types::ServerMonitor;
use crate::core::runner::run_until_signal;
use crate::core::validator::ConfigValidator;
use crate::core::ApplicationConfigLoader;
use edge_proxy::LifecycleMonitor;
use swe_edge_configbuilder::ConfigBuilder as _;
use swe_edge_configbuilder::ConfigBuilderImpl;
use swe_edge_configbuilder::ConfigLoaderFactory;
use swe_observ_metrics::MetricsProvider;

// ── ServerConfigLoader methods ─────────────────────────────────────────────────

impl ServerConfigLoader {
    /// Return a [`ConfigBuilderImpl`] pre-seeded with this crate's package name and version.
    pub fn create_config_builder() -> ConfigBuilderImpl {
        ConfigLoaderFactory::create_config_builder()
            .with_name(env!("CARGO_PKG_NAME"))
            .with_version(env!("CARGO_PKG_VERSION"))
    }

    /// Load config using the default layered chain
    /// (`default.toml` → `application.toml` → env vars).
    ///
    /// The config directory is resolved from `SWE_EDGE_CONFIG_DIR` or
    /// defaults to `config/` relative to the working directory.
    /// Consumer apps should prefer [`ServerConfigLoader::load_config_from`] to supply their
    /// own path explicitly.
    pub fn load_config() -> Result<RuntimeConfig, ConfigError> {
        ApplicationConfigLoader::new().load()
    }

    /// Load config from an explicit directory.
    ///
    /// Identical layer chain to [`ServerConfigLoader::load_config`] but reads
    /// `<dir>/application.toml` instead of relying on env or cwd.
    pub fn load_config_from(
        dir: impl Into<std::path::PathBuf>,
    ) -> Result<RuntimeConfig, ConfigError> {
        ApplicationConfigLoader::with_dir(dir).load()
    }

    /// Load config scoped to a tenant
    /// (`default.toml` → `application.toml` → `tenants/<id>.toml` → env vars).
    pub fn load_tenant_config(tenant_id: &str) -> Result<RuntimeConfig, ConfigError> {
        ApplicationConfigLoader::new().load_for_tenant(tenant_id)
    }

    /// Load tenant config from an explicit directory.
    ///
    /// Reads `<dir>/application.toml` and `<dir>/tenants/<tenant_id>.toml`.
    pub fn load_tenant_config_from(
        tenant_id: &str,
        dir: impl Into<std::path::PathBuf>,
    ) -> Result<RuntimeConfig, ConfigError> {
        ApplicationConfigLoader::with_dir(dir).load_for_tenant(tenant_id)
    }

    /// Load config following the XDG Base Directory specification.
    pub fn load_config_xdg(app_name: &str) -> Result<RuntimeConfig, ConfigError> {
        ApplicationConfigLoader::xdg(app_name).load()
    }

    /// Load tenant config following the XDG Base Directory specification.
    pub fn load_tenant_config_xdg(
        app_name: &str,
        tenant_id: &str,
    ) -> Result<RuntimeConfig, ConfigError> {
        ApplicationConfigLoader::xdg(app_name).load_for_tenant(tenant_id)
    }

    /// Validate a [`RuntimeConfig`] using the built-in [`ConfigValidator`].
    ///
    /// Returns `Err(RuntimeError::StartFailed)` if any field is out of bounds.
    pub fn validate_config(config: &RuntimeConfig) -> Result<(), RuntimeError> {
        ConfigValidator.validate(config)
    }

    /// Load an arbitrary TOML section from the default config chain.
    ///
    /// `key` is a dotted path, e.g. `"observability.tracing"` or
    /// `"application.completion"`.
    pub fn load_section<T>(key: &str) -> Result<T, ConfigError>
    where
        T: serde::de::DeserializeOwned + Default,
    {
        swe_edge_config::load_section(key).map_err(Into::into)
    }

    /// Load an arbitrary TOML section from an explicit config directory.
    pub fn load_section_from<T>(
        key: &str,
        dir: impl Into<std::path::PathBuf>,
    ) -> Result<T, ConfigError>
    where
        T: serde::de::DeserializeOwned + Default,
    {
        swe_edge_config::load_section_from(key, dir).map_err(Into::into)
    }

    /// Load an arbitrary TOML section following the XDG Base Directory chain.
    pub fn load_section_xdg<T>(app_name: &str, key: &str) -> Result<T, ConfigError>
    where
        T: serde::de::DeserializeOwned + Default,
    {
        swe_edge_config::load_section_xdg(app_name, key).map_err(Into::into)
    }
}

// ── ServerMonitor methods ──────────────────────────────────────────────────────

impl ServerMonitor {
    /// Wrap any [`LifecycleMonitor`] with health-state gauge recording.
    ///
    /// After every `health()` call each component's status is emitted as
    /// `edge_component_health` (1.0 = Healthy, 0.5 = Degraded, 0.0 = Unhealthy).
    pub fn observe(
        inner: Arc<dyn LifecycleMonitor>,
        provider: Arc<dyn MetricsProvider>,
    ) -> Arc<dyn LifecycleMonitor> {
        Arc::new(crate::core::monitor::MetricsLifecycleMonitor::new(
            inner, provider,
        ))
    }
}

// ── Runtime SAF methods ────────────────────────────────────────────────────────

impl Runtime {
    /// Assemble a runtime manager from the supplied config, ingress, egress,
    /// and lifecycle monitor.
    pub fn runtime_manager(
        config: RuntimeConfig,
        ingress: Arc<dyn Ingress>,
        egress: Arc<dyn Egress>,
        lifecycle: Arc<dyn LifecycleMonitor>,
    ) -> impl crate::api::runtime_manager::RuntimeManager {
        crate::core::DefaultRuntimeManager::new(config, ingress, egress, lifecycle)
    }

    /// Start the daemon and block until a shutdown signal is received.
    ///
    /// Calls [`RuntimeManager::start`], waits for `SIGTERM` or `SIGINT`
    /// (Ctrl+C on all platforms), then calls [`RuntimeManager::shutdown`].
    /// If shutdown does not complete within `config.shutdown_timeout_secs`,
    /// returns [`RuntimeError::ShutdownTimeout`].
    pub async fn run(
        config: RuntimeConfig,
        ingress: Arc<dyn Ingress>,
        egress: Arc<dyn Egress>,
        lifecycle: Arc<dyn LifecycleMonitor>,
    ) -> RuntimeResult<()> {
        use swe_edge_ingress_grpc::TonicGrpcServer;
        use swe_edge_ingress_http::AxumHttpServer;
        use swe_edge_ingress_verifier::{JwtVerifier, TokenVerifier};
        use tokio::sync::oneshot;

        let timeout_secs = config.shutdown_timeout_secs;
        let http_bind = config.http_bind.clone();
        let grpc_bind = config.grpc_bind.clone();
        let http_tls = config.http_tls.clone();
        let grpc_tls = config.grpc_tls.clone();
        let grpc_allow_unauthenticated = config.grpc_allow_unauthenticated;

        let http_bearer_verifier: Option<Arc<dyn TokenVerifier>> =
            if let Some(ref auth_cfg) = config.http_auth {
                Some(Arc::new(
                    JwtVerifier::from_config(auth_cfg)
                        .map_err(|e| RuntimeError::StartFailed(e.to_string()))?,
                ))
            } else {
                None
            };

        let (http_shutdown_tx, http_shutdown_rx) = oneshot::channel::<()>();
        let http_task = ingress.http().map(|handler| {
            let mut server = AxumHttpServer::new(http_bind, handler);
            if let Some(tls) = http_tls {
                server = server.with_tls(tls);
            }
            if let Some(verifier) = http_bearer_verifier {
                server = server.with_bearer_auth(verifier);
            }
            tokio::spawn(async move {
                let signal = async move {
                    let _ = http_shutdown_rx.await;
                };
                if let Err(e) = server.serve(signal).await {
                    tracing::error!("HTTP server error: {e}");
                }
            })
        });

        let (grpc_shutdown_tx, grpc_shutdown_rx) = oneshot::channel::<()>();
        let grpc_task = ingress.grpc().map(|handler| {
            let mut server = TonicGrpcServer::new(grpc_bind, handler);
            if let Some(tls) = grpc_tls {
                server = server.with_tls(tls);
            }
            if grpc_allow_unauthenticated {
                server = server.allow_unauthenticated(true);
            }
            tokio::spawn(async move {
                let signal = async move {
                    let _ = grpc_shutdown_rx.await;
                };
                if let Err(e) = server.serve(signal).await {
                    tracing::error!("gRPC server error: {e}");
                }
            })
        });

        let mgr = Runtime::runtime_manager(config, ingress, egress, lifecycle);
        let result = run_until_signal(mgr, timeout_secs, Runtime::wait_for_signal()).await;

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
                    tracing::warn!(
                        "could not register SIGTERM handler: {e} — falling back to SIGINT only"
                    );
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
}

/// Return a [`ConfigBuilderImpl`] pre-seeded with this crate's package name and version.
pub fn create_config_builder() -> ConfigBuilderImpl {
    ServerConfigLoader::create_config_builder()
}

/// Load config using the default layered chain.
pub fn load_config() -> Result<RuntimeConfig, ConfigError> {
    ServerConfigLoader::load_config()
}

/// Load config from an explicit directory.
pub fn load_config_from(dir: impl Into<std::path::PathBuf>) -> Result<RuntimeConfig, ConfigError> {
    ServerConfigLoader::load_config_from(dir)
}

/// Load config scoped to a tenant.
pub fn load_tenant_config(tenant_id: &str) -> Result<RuntimeConfig, ConfigError> {
    ServerConfigLoader::load_tenant_config(tenant_id)
}

/// Load tenant config from an explicit directory.
pub fn load_tenant_config_from(
    tenant_id: &str,
    dir: impl Into<std::path::PathBuf>,
) -> Result<RuntimeConfig, ConfigError> {
    ServerConfigLoader::load_tenant_config_from(tenant_id, dir)
}

/// Load config following the XDG Base Directory specification.
pub fn load_config_xdg(app_name: &str) -> Result<RuntimeConfig, ConfigError> {
    ServerConfigLoader::load_config_xdg(app_name)
}

/// Load tenant config following the XDG Base Directory specification.
pub fn load_tenant_config_xdg(
    app_name: &str,
    tenant_id: &str,
) -> Result<RuntimeConfig, ConfigError> {
    ServerConfigLoader::load_tenant_config_xdg(app_name, tenant_id)
}

/// Validate a [`RuntimeConfig`] using the built-in [`ConfigValidator`].
pub fn validate_config(config: &RuntimeConfig) -> Result<(), RuntimeError> {
    ServerConfigLoader::validate_config(config)
}

/// Load an arbitrary TOML section from the default config chain.
pub fn load_section<T>(key: &str) -> Result<T, ConfigError>
where
    T: serde::de::DeserializeOwned + Default,
{
    ServerConfigLoader::load_section(key)
}

/// Load an arbitrary TOML section from an explicit config directory.
pub fn load_section_from<T>(key: &str, dir: impl Into<std::path::PathBuf>) -> Result<T, ConfigError>
where
    T: serde::de::DeserializeOwned + Default,
{
    ServerConfigLoader::load_section_from(key, dir)
}

/// Load an arbitrary TOML section following the XDG Base Directory chain.
pub fn load_section_xdg<T>(app_name: &str, key: &str) -> Result<T, ConfigError>
where
    T: serde::de::DeserializeOwned + Default,
{
    ServerConfigLoader::load_section_xdg(app_name, key)
}

/// Wrap any [`LifecycleMonitor`] with health-state gauge recording.
pub fn observe_lifecycle_monitor(
    inner: Arc<dyn LifecycleMonitor>,
    provider: Arc<dyn MetricsProvider>,
) -> Arc<dyn LifecycleMonitor> {
    ServerMonitor::observe(inner, provider)
}

/// Assemble a runtime manager from the supplied config, ingress, egress,
/// and lifecycle monitor.
pub fn runtime_manager(
    config: RuntimeConfig,
    ingress: Arc<dyn Ingress>,
    egress: Arc<dyn Egress>,
    lifecycle: Arc<dyn LifecycleMonitor>,
) -> impl crate::api::runtime_manager::RuntimeManager {
    Runtime::runtime_manager(config, ingress, egress, lifecycle)
}

/// Start the daemon and block until a shutdown signal is received.
pub async fn run(
    config: RuntimeConfig,
    ingress: Arc<dyn Ingress>,
    egress: Arc<dyn Egress>,
    lifecycle: Arc<dyn LifecycleMonitor>,
) -> RuntimeResult<()> {
    Runtime::run(config, ingress, egress, lifecycle).await
}
