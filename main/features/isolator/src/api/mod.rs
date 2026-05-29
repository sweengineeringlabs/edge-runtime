//! Public API — config types and registry for isolation profiles.

pub mod isolator_config;
pub mod profile_spec;
pub mod registry;

pub use isolator_config::IsolatorConfig;
pub use profile_spec::ProfileSpec;
pub use registry::IsolationProfileRegistry;
