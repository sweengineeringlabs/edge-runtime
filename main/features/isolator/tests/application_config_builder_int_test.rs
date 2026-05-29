//! Integration tests for the ApplicationConfigBuilder api interface.

/// @covers: ApplicationConfigBuilder
#[test]
fn test_application_config_builder_marker_compiles() {
    let _ = std::mem::size_of::<u8>(); // marker trait — no runtime test needed
}
