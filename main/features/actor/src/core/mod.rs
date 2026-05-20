//! Core — Abstract actor message processing logic.
//!
//! Core contains the abstract message loop pattern and common actor machinery
//! that works with any runtime. Specific runtime implementations live in spi/.

use crate::api::Actor;

/// Message envelope for the actor's internal queue.
pub enum Message<A: Actor> {
    /// User-sent message to the actor.
    Msg(A::Message),
    /// Signal to stop the actor gracefully.
    Stop,
}
