//! Runner contract — start, await signal, drain.

use crate::api::runtime::errors::runtime_result::RuntimeResult;
use crate::api::runtime::types::runtime::Runtime;
use crate::api::runtime::types::runtime_builder::RuntimeBuilder;
use crate::api::runtime::types::runtime_builder_serve::RuntimeBuilderServe;
use crate::api::runtime::types::server_config_loader::ServerConfigLoader;
use crate::api::runtime::types::server_monitor::ServerMonitor;
use crate::api::runtime::types::tracing_initializer::TracingInitializer;

/// Drives a [`RuntimeManager`] through start → signal → shutdown.
pub trait Runner: Send + Sync {
    /// Drive the runtime through start → signal → shutdown.
    fn run(&self) -> RuntimeResult<()>;

    /// Construct a fresh [`RuntimeBuilder`] via [`Runtime::builder`].
    fn new_builder() -> RuntimeBuilder
    where
        Self: Sized,
    {
        Runtime::builder()
    }

    /// Return the zero-size [`Runtime`] entry-point.
    fn runtime_entry() -> Runtime
    where
        Self: Sized,
    {
        Runtime
    }

    /// Return the zero-size [`ServerConfigLoader`] factory.
    fn server_config_loader() -> ServerConfigLoader
    where
        Self: Sized,
    {
        ServerConfigLoader
    }

    /// Return the zero-size [`ServerMonitor`] factory.
    fn server_monitor() -> ServerMonitor
    where
        Self: Sized,
    {
        ServerMonitor
    }

    /// Return the zero-size [`TracingInitializer`] factory.
    fn tracing_initializer() -> TracingInitializer
    where
        Self: Sized,
    {
        TracingInitializer
    }

    /// Return the zero-size [`RuntimeBuilderServe`] marker.
    fn builder_serve_marker() -> RuntimeBuilderServe
    where
        Self: Sized,
    {
        RuntimeBuilderServe
    }
}
