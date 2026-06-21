//! SAF factory functions for the gRPC server crate.

/// Return a config builder pre-seeded with this crate's package name and version.
pub fn create_config_builder() -> swe_edge_configbuilder::ConfigBuilderImpl {
    let mut b = swe_edge_configbuilder::ConfigBuilderImpl::new();
    b = b.with_name(env!("CARGO_PKG_NAME"));
    b = b.with_version(env!("CARGO_PKG_VERSION"));
    b
}
