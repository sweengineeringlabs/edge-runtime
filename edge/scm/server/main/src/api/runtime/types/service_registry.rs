//! `ServiceRegistry` — stable egress client registry handed to handlers at construction time.

use std::sync::Arc;

use swe_edge_egress_grpc::GrpcEgress;
use swe_edge_egress_http::HttpEgress;

/// Holds egress clients that handlers may use to make outbound calls.
///
/// Constructed by [`RuntimeBuilder::build_registry`](crate::RuntimeBuilder::build_registry) and passed to
/// handler constructors at startup — not per-request.  Share it via
/// `Arc<ServiceRegistry>`.
pub struct ServiceRegistry {
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

impl ServiceRegistry {
    /// Construct a registry from an HTTP egress client and an optional gRPC client.
    pub fn new(http: Arc<dyn HttpEgress>, grpc: Option<Arc<dyn GrpcEgress>>) -> Self {
        Self {
            http,
            grpc,
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

    /// Return the HTTP egress client.
    pub fn http(&self) -> &Arc<dyn HttpEgress> {
        &self.http
    }

    /// Return the gRPC egress client, if one was registered.
    pub fn grpc(&self) -> Option<&Arc<dyn GrpcEgress>> {
        self.grpc.as_ref()
    }

    /// Attach a subprocess runner, consumed by [`RuntimeBuilder::build_registry`](crate::RuntimeBuilder::build_registry).
    #[cfg(feature = "subprocess")]
    pub fn with_subprocess(
        mut self,
        runner: Arc<dyn swe_edge_egress_subprocess::SubprocessRunner>,
    ) -> Self {
        self.subprocess = Some(runner);
        self
    }

    /// Return the subprocess runner, if one was registered.
    #[cfg(feature = "subprocess")]
    pub fn subprocess(&self) -> Option<&Arc<dyn swe_edge_egress_subprocess::SubprocessRunner>> {
        self.subprocess.as_ref()
    }

    /// Attach a CLI runner, consumed by [`RuntimeBuilder::build_registry`](crate::RuntimeBuilder::build_registry).
    #[cfg(feature = "cli")]
    pub fn with_cli_runner(mut self, runner: Arc<dyn swe_edge_runtime_cli::CliRunner>) -> Self {
        self.cli_runner = Some(runner);
        self
    }

    /// Return the CLI runner, if one was registered.
    #[cfg(feature = "cli")]
    pub fn cli_runner(&self) -> Option<&Arc<dyn swe_edge_runtime_cli::CliRunner>> {
        self.cli_runner.as_ref()
    }

    /// Attach a runtime HTTP ingress handler, consumed by [`RuntimeBuilder::build_registry`](crate::RuntimeBuilder::build_registry).
    #[cfg(feature = "http")]
    pub fn with_http_ingress(
        mut self,
        handler: Arc<dyn swe_edge_runtime_http::HttpIngress>,
    ) -> Self {
        self.http_ingress = Some(handler);
        self
    }

    /// Return the runtime HTTP ingress handler, if one was registered.
    #[cfg(feature = "http")]
    pub fn http_ingress(&self) -> Option<&Arc<dyn swe_edge_runtime_http::HttpIngress>> {
        self.http_ingress.as_ref()
    }

    /// Attach a runtime gRPC ingress handler, consumed by [`RuntimeBuilder::build_registry`](crate::RuntimeBuilder::build_registry).
    #[cfg(feature = "grpc")]
    pub fn with_grpc_ingress(
        mut self,
        handler: Arc<dyn swe_edge_runtime_grpc::GrpcIngress>,
    ) -> Self {
        self.grpc_ingress = Some(handler);
        self
    }

    /// Return the runtime gRPC ingress handler, if one was registered.
    #[cfg(feature = "grpc")]
    pub fn grpc_ingress(&self) -> Option<&Arc<dyn swe_edge_runtime_grpc::GrpcIngress>> {
        self.grpc_ingress.as_ref()
    }
}
