//! API layer — runtime trait contracts and public types.

pub(crate) mod application_config_builder;
pub(crate) mod architecture_config_builder;
pub(crate) mod composite;
pub(crate) mod composite_ingress;
pub(crate) mod config;
pub(crate) mod config_loader;
pub(crate) mod default_config_builder;
pub(crate) mod egress;
pub(crate) mod error;
pub(crate) mod ingress;
pub(crate) mod json_codec;
pub(crate) mod metrics_exporter;
pub(crate) mod metrics_handler;
pub(crate) mod monitor;
pub(crate) mod observability;
pub(crate) mod runner;
pub(crate) mod runtime;
pub(crate) mod runtime_manager;
pub(crate) mod service_registry;
pub(crate) mod traits;
pub(crate) mod types;
pub(crate) mod validator;
