//! SweEdgeRuntimeResourcePolicy api interface — counterpart to core/swe/default/.
//!
//! Provides the primary service trait re-export for the default implementation
//! in [`crate::core::swe::default::swe_edge_runtime_resource_policy`].

/// Re-export of the primary service trait for the default implementation.
///
/// Implementors: [`crate::core::swe::default::swe_edge_runtime_resource_policy::DefaultSweEdgeRuntimeResourcePolicy`].
pub use crate::api::traits::swe_edge_runtime_resource_policy::SweEdgeRuntimeResourcePolicy;
