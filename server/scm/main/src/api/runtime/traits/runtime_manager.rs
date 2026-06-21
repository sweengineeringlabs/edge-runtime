//! RuntimeManager trait — owns the full process lifecycle.

use futures::future::BoxFuture;

use crate::api::runtime::errors::runtime_result::RuntimeResult;
use crate::api::runtime::types::component_health::ComponentHealth;
use crate::api::runtime::types::runtime_health::RuntimeHealth;
use crate::api::runtime::types::runtime_status::RuntimeStatus;
use crate::api::runtime::types::service_registry::ServiceRegistry;
use crate::api::runtime::types::service_registry_builder::ServiceRegistryBuilder;

/// Manages the full process lifecycle: start, shutdown, and health.
///
/// Implementations wire ingress servers, the controller, and egress
/// adapters into a single runtime that can be started and stopped
/// cleanly. Designed to integrate with systemd via `sd_notify`.
pub trait RuntimeManager: Send + Sync {
    /// Start all ingress servers and background tasks.
    ///
    /// Resolves when the runtime is fully started and ready to serve
    /// traffic. Implementations should emit `sd_notify READY=1` here.
    fn start(&self) -> BoxFuture<'_, RuntimeResult<()>>;

    /// Gracefully shut down: drain in-flight requests, stop servers,
    /// release resources. Implementations should emit
    /// `sd_notify STOPPING=1` before beginning teardown.
    fn shutdown(&self) -> BoxFuture<'_, RuntimeResult<()>>;

    /// Aggregate health across all wired components.
    fn health(&self) -> BoxFuture<'_, RuntimeHealth>;

    /// Current lifecycle status derived from the last health snapshot.
    fn runtime_status(&self) -> RuntimeStatus {
        RuntimeStatus::Running
    }

    /// Per-subsystem health snapshots derived from the last health snapshot.
    fn list_components(&self) -> Vec<ComponentHealth> {
        vec![]
    }

    /// Access the service registry if one was wired at construction time.
    fn service_registry(&self) -> Option<&ServiceRegistry> {
        None
    }

    /// Return a builder pre-loaded with the HTTP egress from the wired registry.
    ///
    /// Returns `None` when no service registry is available.
    fn service_registry_builder(&self) -> Option<ServiceRegistryBuilder> {
        self.service_registry()
            .map(|r| ServiceRegistryBuilder::new(std::sync::Arc::clone(r.http())))
    }
}
