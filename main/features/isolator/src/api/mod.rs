//! Public API — traits, config types, and registry for isolation profiles.

pub mod error;
pub mod noop;
pub mod profile_resolver;
pub mod seccomp;
pub mod swe;
pub mod traits;
pub mod types;

pub use error::Error;
pub use traits::SweEdgeRuntimeIsolator;
pub use traits::Validator;
pub use types::DefaultSweEdgeRuntimeIsolatorImpl;
pub use types::DefaultSweEdgeRuntimeIsolatorValidatorImpl;
pub use types::IsolationProfileRegistry;
pub use types::IsolatorConfig;
pub use types::IsolatorSvc;
pub use types::ProfileSpec;
pub use types::SweEdgeRuntimeIsolatorFactory;
