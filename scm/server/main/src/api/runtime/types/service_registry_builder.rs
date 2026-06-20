//! `ServiceRegistryBuilder` — fluent builder for [`ServiceRegistry`].

use std::sync::Arc;

use swe_edge_egress_grpc::GrpcEgress;
use swe_edge_egress_http::HttpEgress;

use super::service_registry::ServiceRegistry;

/// Fluent builder for [`ServiceRegistry`].
///
/// Obtain one via [`crate::api::RuntimeManager::service_registry_builder`]
/// or construct directly with [`ServiceRegistryBuilder::new`].
pub struct ServiceRegistryBuilder {
    http: Arc<dyn HttpEgress>,
    grpc: Option<Arc<dyn GrpcEgress>>,
    #[cfg(feature = "subprocess")]
    subprocess: Option<Arc<dyn swe_edge_egress_subprocess::SubprocessRunner>>,
    #[cfg(feature = "cli")]
    cli_runner: Option<Arc<dyn swe_edge_runtime_cli::CliRunner>>,
    #[cfg(feature = "http")]
    http_ingress: Option<Arc<dyn swe_edge_runtime_http::HttpIngress>>,
    #[cfg(feature = "grpc")]
    grpc_ingress: Option<Arc<dyn swe_edge_runtime_grpc::GrpcIngress>>,
}

impl ServiceRegistryBuilder {
    /// Begin building a registry with the required HTTP egress client.
    pub fn new(http: Arc<dyn HttpEgress>) -> Self {
        Self {
            http,
            grpc: None,
            #[cfg(feature = "subprocess")]
            subprocess: None,
            #[cfg(feature = "cli")]
            cli_runner: None,
            #[cfg(feature = "http")]
            http_ingress: None,
            #[cfg(feature = "grpc")]
            grpc_ingress: None,
        }
    }

    /// Attach an optional gRPC egress client.
    pub fn grpc(mut self, client: Arc<dyn GrpcEgress>) -> Self {
        self.grpc = Some(client);
        self
    }

    /// Attach a subprocess runner.
    #[cfg(feature = "subprocess")]
    pub fn subprocess(
        mut self,
        runner: Arc<dyn swe_edge_egress_subprocess::SubprocessRunner>,
    ) -> Self {
        self.subprocess = Some(runner);
        self
    }

    /// Attach a CLI runner.
    #[cfg(feature = "cli")]
    pub fn cli_runner(mut self, runner: Arc<dyn swe_edge_runtime_cli::CliRunner>) -> Self {
        self.cli_runner = Some(runner);
        self
    }

    /// Attach a runtime HTTP ingress handler.
    #[cfg(feature = "http")]
    pub fn http_ingress(mut self, handler: Arc<dyn swe_edge_runtime_http::HttpIngress>) -> Self {
        self.http_ingress = Some(handler);
        self
    }

    /// Attach a runtime gRPC ingress handler.
    #[cfg(feature = "grpc")]
    pub fn grpc_ingress(mut self, handler: Arc<dyn swe_edge_runtime_grpc::GrpcIngress>) -> Self {
        self.grpc_ingress = Some(handler);
        self
    }

    /// Consume the builder and produce a [`ServiceRegistry`].
    pub fn build(self) -> ServiceRegistry {
        let reg = ServiceRegistry::new(self.http, self.grpc);
        #[cfg(feature = "subprocess")]
        let reg = match self.subprocess {
            Some(r) => reg.with_subprocess(r),
            None => reg,
        };
        #[cfg(feature = "cli")]
        let reg = match self.cli_runner {
            Some(r) => reg.with_cli_runner(r),
            None => reg,
        };
        #[cfg(feature = "http")]
        let reg = match self.http_ingress {
            Some(h) => reg.with_http_ingress(h),
            None => reg,
        };
        #[cfg(feature = "grpc")]
        let reg = match self.grpc_ingress {
            Some(h) => reg.with_grpc_ingress(h),
            None => reg,
        };
        reg
    }
}
