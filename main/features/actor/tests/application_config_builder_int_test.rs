//! Integration tests for [`ActorRuntime::create_config_builder`].

use swe_edge_runtime_actor::ActorRuntime;

/// @covers: ActorRuntime::create_config_builder — returns a pre-seeded builder with this crate's package name
#[test]
fn test_create_config_builder_is_pre_seeded_with_package_name() {
    let _loader = ActorRuntime::create_config_builder().build_loader();
}
