//! [`MessageStream`] — ordered stream of messages from a broker subscription.

use std::pin::Pin;

use futures::Stream;

use crate::api::broker::broker_error::BrokerError;
use crate::api::broker::message::message::Message;

/// An ordered stream of messages received from a [`crate::MessageBroker`] subscription.
pub type MessageStream = Pin<Box<dyn Stream<Item = Result<Message, BrokerError>> + Send>>;
