//! Public API — resource limit types, policy, and config.

pub mod error;
pub mod policy_runner;
pub mod resolver;
pub mod swe;
pub mod traits;
pub mod types;

pub use error::Error;
pub use error::ResourcePolicyError;
pub use types::ResourceLimits;
pub use types::ResourceLimitsResolver;
pub use types::ResourcePolicy;
pub use types::ResourcePolicyConfig;
pub use types::ResourcePolicyRunner;
pub use traits::SweEdgeRuntimeResourcePolicy;
pub use traits::Validator;
pub use types::SweEdgeRuntimeResourcePolicyFactory;
pub use types::DefaultSweEdgeRuntimeResourcePolicyImpl;
pub use types::DefaultSweEdgeRuntimeResourcePolicyValidatorImpl;
pub use types::PolicySvc;
