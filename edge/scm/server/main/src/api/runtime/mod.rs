//! Runtime theme — runtime entry-point, builder, manager, config, health, errors.

pub(crate) mod errors;
pub(crate) mod manager;
pub(crate) mod runner;
pub(crate) mod runtime_builder_serve;
pub(crate) mod traits;
pub(crate) mod types;

pub use errors::{RuntimeError, RuntimeResult};
pub use traits::{ConfigValidator, Runner, RuntimeManager, Validator};
pub use types::runtime_config::RuntimeConfig;
pub use types::{
    ComponentHealth, Runtime, RuntimeBuilder, RuntimeBuilderServe, RuntimeHealth, RuntimeStatus,
    ServerConfigLoader, ServerMonitor, ServiceRegistry, ServiceRegistryBuilder, TracingInitializer,
};
