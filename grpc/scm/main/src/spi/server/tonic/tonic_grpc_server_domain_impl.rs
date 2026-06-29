//! GrpcServerManage trait impl for TonicGrpcServer.
use std::sync::Arc;
use std::time::Duration;

use edge_domain_security::PemTlsConfig;
use swe_edge_ingress_grpc::{
    AuditSink, CompressionMode, GrpcIngress, GrpcIngressInterceptorChain, HealthService,
    NoopAuditSink,
};

use crate::api::{
    GrpcServerConfig, GrpcServerConfigError, GrpcServerManage, TonicGrpcServer,
    DEFAULT_KEEPALIVE_INTERVAL, DEFAULT_KEEPALIVE_TIMEOUT, MAX_MESSAGE_BYTES,
};

impl GrpcServerManage for TonicGrpcServer {
    fn new(bind: impl Into<String>, handler: Arc<dyn GrpcIngress>) -> Self {
        Self {
            bind: bind.into(),
            handler,
            max_bytes: MAX_MESSAGE_BYTES,
            max_concurrent_streams: 100,
            tls: None,
            interceptors: GrpcIngressInterceptorChain::new(),
            compression: CompressionMode::None,
            allow_unauthenticated: false,
            audit_sink: Arc::new(NoopAuditSink),
            enable_reflection: false,
            health_service: Some(Arc::new(HealthService::new())),
            auto_trace_context: true,
            keepalive_interval: Some(DEFAULT_KEEPALIVE_INTERVAL),
            keepalive_timeout: DEFAULT_KEEPALIVE_TIMEOUT,
        }
    }

    fn from_config(
        config: &GrpcServerConfig,
        handler: Arc<dyn GrpcIngress>,
    ) -> Result<Self, GrpcServerConfigError> {
        if config.tls_required && config.tls.is_none() {
            return Err(GrpcServerConfigError::TlsRequiredButMissing);
        }
        Ok(Self {
            bind: config.bind.to_string(),
            handler,
            max_bytes: config.max_message_bytes,
            max_concurrent_streams: config.max_concurrent_streams,
            tls: config.tls.clone(),
            interceptors: GrpcIngressInterceptorChain::new(),
            compression: config.compression,
            allow_unauthenticated: config.allow_unauthenticated,
            audit_sink: Arc::new(NoopAuditSink),
            enable_reflection: config.enable_reflection,
            health_service: Some(Arc::new(HealthService::new())),
            auto_trace_context: true,
            keepalive_interval: config.keepalive_interval_secs.map(Duration::from_secs),
            keepalive_timeout: Duration::from_secs(config.keepalive_timeout_secs),
        })
    }

    fn enable_reflection(mut self, enable: bool) -> Self {
        self.enable_reflection = enable;
        self
    }

    fn is_reflection_enabled(&self) -> bool {
        self.enable_reflection
    }

    fn with_audit_sink(mut self, sink: Arc<dyn AuditSink>) -> Self {
        self.audit_sink = sink;
        self
    }

    fn allow_unauthenticated(mut self, allow: bool) -> Self {
        self.allow_unauthenticated = allow;
        self
    }

    fn with_max_message_size(mut self, size: usize) -> Self {
        self.max_bytes = size;
        self
    }

    fn with_max_concurrent_streams(mut self, streams: u32) -> Self {
        self.max_concurrent_streams = streams;
        self
    }

    fn with_interceptors(mut self, chain: GrpcIngressInterceptorChain) -> Self {
        self.interceptors = chain;
        self
    }

    fn with_compression(mut self, mode: CompressionMode) -> Self {
        self.compression = mode;
        self
    }

    fn with_tls(mut self, config: PemTlsConfig) -> Self {
        self.tls = Some(config);
        self
    }

    fn with_keepalive(mut self, interval: Duration, timeout: Duration) -> Self {
        self.keepalive_interval = Some(interval);
        self.keepalive_timeout = timeout;
        self
    }

    fn without_keepalive(mut self) -> Self {
        self.keepalive_interval = None;
        self
    }

    fn keepalive_interval(&self) -> Option<Duration> {
        self.keepalive_interval
    }

    fn keepalive_timeout(&self) -> Duration {
        self.keepalive_timeout
    }

    fn without_trace_context(mut self) -> Self {
        self.auto_trace_context = false;
        self
    }

    fn without_health_service(mut self) -> Self {
        self.health_service = None;
        self
    }

    fn health_service(&self) -> Option<&Arc<HealthService>> {
        self.health_service.as_ref()
    }

    fn with_health_service(mut self, hs: Arc<HealthService>) -> Self {
        self.health_service = Some(hs);
        self
    }

    fn bind_addr(&self) -> &str {
        &self.bind
    }

    fn max_message_size(&self) -> usize {
        self.max_bytes
    }

    fn max_concurrent_streams(&self) -> u32 {
        self.max_concurrent_streams
    }

    fn tls_config(&self) -> Option<&PemTlsConfig> {
        self.tls.as_ref()
    }

    fn compression_mode(&self) -> CompressionMode {
        self.compression
    }

    fn is_unauthenticated_allowed(&self) -> bool {
        self.allow_unauthenticated
    }

    fn has_trace_context(&self) -> bool {
        self.auto_trace_context
    }

    fn audit_sink_ref(&self) -> &Arc<dyn AuditSink> {
        &self.audit_sink
    }

    fn interceptor_chain(&self) -> &GrpcIngressInterceptorChain {
        &self.interceptors
    }
}
