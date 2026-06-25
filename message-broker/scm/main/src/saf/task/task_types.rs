//! Inherent impls for task value types.

use std::collections::HashMap;

use bytes::Bytes;
use futures::future::BoxFuture;
use uuid::Uuid;

use crate::api::QueueError;
use crate::api::Task;
use crate::api::TaskHandle;
use crate::api::TaskHandleBuilder;
use crate::api::TaskId;

impl TaskId {
    /// Generate a new random task ID.
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    /// Create a TaskId from a UUID.
    pub fn from_uuid(uuid: Uuid) -> Self {
        Self(uuid)
    }

    /// Get the underlying UUID.
    pub fn as_uuid(&self) -> Uuid {
        self.0
    }
}

impl Task {
    /// Construct a task from raw bytes with no headers.
    pub fn new(payload: impl Into<Bytes>) -> Self {
        Self {
            id: TaskId::new(),
            payload: payload.into(),
            headers: HashMap::new(),
        }
    }

    /// Construct a task with headers.
    pub fn with_headers(payload: impl Into<Bytes>, headers: HashMap<String, String>) -> Self {
        Self {
            id: TaskId::new(),
            payload: payload.into(),
            headers,
        }
    }

    /// Construct a task with a specific ID.
    pub fn with_id(id: TaskId, payload: impl Into<Bytes>) -> Self {
        Self {
            id,
            payload: payload.into(),
            headers: HashMap::new(),
        }
    }
}

impl TaskHandle {
    /// Create a [`TaskHandle`] from the dequeued task's primitives and its ack/nack futures.
    pub fn new(
        task_id: TaskId,
        payload: Bytes,
        headers: HashMap<String, String>,
        ack: BoxFuture<'static, Result<(), QueueError>>,
        nack: BoxFuture<'static, Result<(), QueueError>>,
    ) -> Self {
        Self {
            task_id,
            payload,
            headers,
            ack,
            nack,
        }
    }
}

impl TaskHandleBuilder {
    /// Create a builder with all required fields.
    pub fn new(
        task_id: TaskId,
        payload: Bytes,
        ack: BoxFuture<'static, Result<(), QueueError>>,
        nack: BoxFuture<'static, Result<(), QueueError>>,
    ) -> Self {
        Self {
            task_id,
            payload,
            headers: HashMap::new(),
            ack,
            nack,
        }
    }

    /// Set the optional key-value metadata headers for the task handle.
    pub fn headers(mut self, headers: HashMap<String, String>) -> Self {
        self.headers = headers;
        self
    }

    /// Consume the builder and produce a [`TaskHandle`].
    pub fn build(self) -> TaskHandle {
        TaskHandle::new(
            self.task_id,
            self.payload,
            self.headers,
            self.ack,
            self.nack,
        )
    }
}
