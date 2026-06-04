//! Core — Abstract actor message processing logic.
//!
//! Core contains the default implementations of api/ traits that apply
//! across all runtime backends. Specific runtime implementations live in spi/.

mod validator;

#[expect(
    unused_imports,
    reason = "SEA core/ anchor — wired up when validator integrates into spawn"
)]
pub(crate) use validator::DefaultActorValidator;
