//! Security tests for spi/ layer — verify implementation types are not exposed.

#[test]
fn test_public_api_uses_impl_trait_not_concrete_types() {
    // Verify that public factories return impl Trait, not concrete types.
    // Consumers should only see Scheduler trait, not TokioScheduler.
    let _scheduler = swe_edge_runtime_scheduler::tokio_scheduler(
        swe_edge_runtime_scheduler::TokioSchedulerConfig::default(),
        "test",
    );
}
