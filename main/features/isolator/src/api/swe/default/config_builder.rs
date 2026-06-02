//! `DefaultConfigBuilder` api interface — contract for the default config builder.

/// Marker trait for the default configuration builder contract.
///
/// Implemented by `core::swe::default::config_builder::DefaultConfigBuilder`.
#[expect(
    dead_code,
    reason = "SEA api/ anchor — exported for consumers, not used internally"
)]
pub trait ConfigBuilder {}
