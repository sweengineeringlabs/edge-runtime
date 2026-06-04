//! Error types for swe_edge_runtime_resource_policy.

#[allow(clippy::module_inception)]
pub mod error;
pub mod resource_policy_error;

pub use error::Error;
pub use resource_policy_error::ResourcePolicyError;
