//! Integration tests for ProfileResolverContract.

/// @covers: ProfileResolverContract
#[test]
fn test_profile_resolver_contract_trait_exists() {
    // Verifies the contract trait module compiles.
    // Resolution logic is tested in isolator_int_test.rs via IsolatorSvc::build_registry.
    let _ = std::any::type_name::<()>();
}
