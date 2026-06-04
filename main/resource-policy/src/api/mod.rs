//! Public API — resource limit types, policy, and config.

pub mod error;
pub mod policy;
pub mod resolver;
pub mod swe;
pub mod traits;
pub mod types;

pub use error::Error;
pub use error::ResourcePolicyError;
pub use traits::SweEdgeRuntimeResourcePolicy;
pub use traits::Validator;
pub use types::NoopRuntimeResourcePolicy;
pub use types::PolicySvc;
pub use types::ResourceLimits;
pub use types::ResourceLimitsBuilder;
pub use types::ResourceLimitsResolver;
pub use types::ResourcePolicy;
pub use types::ResourcePolicyBuilder;
pub use types::ResourcePolicyConfig;
pub use types::ResourcePolicyRunner;
pub use types::SweEdgeRuntimeResourcePolicyFactory;
