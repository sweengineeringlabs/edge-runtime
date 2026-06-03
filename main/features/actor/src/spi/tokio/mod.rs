//! Tokio runtime implementation of the actor mailbox.

mod actor_handle;
mod mailbox;
mod stop_handle;

pub(crate) use actor_handle::TokioActorHandle;
#[cfg_attr(
    not(test),
    expect(
        unused_imports,
        reason = "SEA spi/ anchor — TokioMailbox used in tests and SAF wiring"
    )
)]
pub(crate) use mailbox::TokioMailbox;
pub(crate) use stop_handle::TokioStopHandle;
