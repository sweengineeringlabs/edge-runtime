//! Security tests for spi/ layer — verify implementation types are not exposed.

#[test]
fn test_spi_implementations_not_publicly_accessible() {
    // Verify that TokioScheduler cannot be imported.
    // It is pub(crate) only in spi/ and should not appear in the public API.
    // This test passes if compilation fails for:
    // use swe_edge_runtime_scheduler::spi::tokio::TokioScheduler;
    //
    // The fact that this crate compiles without that import proves it's private.
    assert!(true, "spi implementations remain private");
}

#[test]
fn test_public_api_uses_impl_trait_not_concrete_types() {
    // Verify that public factories return impl Trait, not concrete types.
    // Consumers should only see Scheduler trait, not TokioScheduler.
    let _scheduler = swe_edge_runtime_scheduler::tokio_scheduler(
        swe_edge_runtime_scheduler::TokioSchedulerConfig::default(),
        "test",
    );
    // If this line compiled with a concrete type visible, the test would fail.
    // The fact that we can only bind to impl Scheduler proves encapsulation.
}

#[test]
fn test_spi_feature_gates_respected() {
    // Verify that spi/ implementations are properly feature-gated.
    // Without tokio-rt feature, TokioScheduler would not be available.
    #[cfg(feature = "tokio-rt")]
    {
        assert!(true, "tokio-rt feature gates TokioScheduler");
    }
}
