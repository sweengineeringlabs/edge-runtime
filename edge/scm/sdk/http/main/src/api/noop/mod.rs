//! No-op HTTP ingress and validator contract types — declared here for `core/noop` correspondence.
mod noop_http_ingress;
mod noop_validator;

pub use noop_http_ingress::NoopHttpIngress;
pub use noop_validator::NoopValidator;
