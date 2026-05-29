//! Message envelope for the actor's internal queue.

use crate::api::actor::actor::Actor;

/// Message envelope for the actor's internal queue.
pub enum Message<A: Actor> {
    /// User-sent message to the actor.
    Msg(A::Message),
    /// Signal to stop the actor gracefully.
    Stop,
}
