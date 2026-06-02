//! Interface contract for the default actor validator.
//!
//! The concrete implementation lives in `core/validator/default_actor_validator.rs`.
//! This module re-exports the `Validator` trait so `core/` implementations
//! can implement against the public interface without bypassing the api/ boundary.
