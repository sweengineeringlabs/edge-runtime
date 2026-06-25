//! [`InMemoryMessageBroker`] — tokio broadcast-channel backed broker.

use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::{broadcast, RwLock};

use crate::api::broker::Message;

/// In-memory pub/sub broker backed by [`tokio::sync::broadcast`].
///
/// Topics are created lazily on first subscription.  Multiple handles to the
/// same broker share a single channel map via an internal `Arc`, so cloning
/// this struct produces another handle to the same broker.
///
/// Requires the `tokio-rt` feature.
#[derive(Clone)]
pub struct InMemoryMessageBroker {
    pub(crate) channels: Arc<RwLock<HashMap<String, broadcast::Sender<Message>>>>,
}
