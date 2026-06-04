//! Security tests for spi/ layer — verify implementation types are not exposed.
//!
//! The public API exports only trait objects and factories returning `impl Trait`.
//! Implementation types (InMemoryMessageBroker, NatsMessageBroker, etc.) are pub(crate) and
//! verified not to appear in the public module tree. Compile-time privacy checking
//! is the authoritative test; no runtime assertion is needed.
