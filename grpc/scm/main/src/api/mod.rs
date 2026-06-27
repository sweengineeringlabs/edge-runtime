//! Public contract declarations — server traits, types, and errors.
mod noop;
mod server;
mod tls;

pub use noop::{NoopGrpcIngress, NoopGrpcValidator, Validator, ValidatorSvc};
pub use tls::TlsSvc;
pub use server::{
    GrpcServer, GrpcServerConfig, GrpcServerConfigBuilder, GrpcServerConfigError, GrpcServerError,
    GrpcServerObserver, GrpcServerObserverSvc, GrpcServerSvc, StatusCodeConverter, TonicGrpcServer,
    TonicGrpcServerBuilder, DEFAULT_KEEPALIVE_INTERVAL, DEFAULT_KEEPALIVE_INTERVAL_SECS,
    DEFAULT_KEEPALIVE_TIMEOUT, DEFAULT_KEEPALIVE_TIMEOUT_SECS, DEFAULT_MAX_CONCURRENT_STREAMS,
    DEFAULT_MAX_MESSAGE_BYTES, MAX_MESSAGE_BYTES, MISSING_AUTHORIZATION_INTERCEPTOR_MSG,
    REFLECTION_ENABLED_WARN_MSG, SANITIZED_INTERNAL_MSG,
};
