//! Integration tests for [`TaskHandleBuilder`].
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::collections::HashMap;

use bytes::Bytes;
use swe_edge_runtime_message_broker::{QueueError, TaskHandleBuilder, TaskId};

/// @covers: TaskHandleBuilder::new
#[test]
fn test_new_produces_builder_with_empty_headers_happy() {
    let id = TaskId::new();
    let payload = Bytes::from_static(b"hello");
    let ack: futures::future::BoxFuture<'static, Result<(), QueueError>> =
        Box::pin(async { Ok(()) });
    let nack: futures::future::BoxFuture<'static, Result<(), QueueError>> =
        Box::pin(async { Ok(()) });
    let handle = TaskHandleBuilder::new(id, payload, ack, nack).build();
    assert!(
        handle.headers.is_empty(),
        "builder default headers must be empty"
    );
}

/// @covers: TaskHandleBuilder::new
#[test]
fn test_new_with_empty_payload_builds_valid_handle_edge() {
    let id = TaskId::new();
    let payload = Bytes::new();
    let ack: futures::future::BoxFuture<'static, Result<(), QueueError>> =
        Box::pin(async { Ok(()) });
    let nack: futures::future::BoxFuture<'static, Result<(), QueueError>> =
        Box::pin(async { Ok(()) });
    let handle = TaskHandleBuilder::new(id, payload, ack, nack).build();
    assert!(
        handle.payload.is_empty(),
        "builder must accept empty payload"
    );
}

/// @covers: TaskHandleBuilder::headers
#[test]
fn test_headers_fluent_setter_stores_headers_happy() {
    let id = TaskId::new();
    let payload = Bytes::from_static(b"data");
    let mut headers = HashMap::new();
    headers.insert("x-trace".to_owned(), "abc".to_owned());
    let ack: futures::future::BoxFuture<'static, Result<(), QueueError>> =
        Box::pin(async { Ok(()) });
    let nack: futures::future::BoxFuture<'static, Result<(), QueueError>> =
        Box::pin(async { Ok(()) });
    let handle = TaskHandleBuilder::new(id, payload, ack, nack)
        .headers(headers.clone())
        .build();
    assert_eq!(
        handle.headers, headers,
        "headers setter must store the provided map"
    );
}

/// @covers: TaskHandleBuilder::headers
#[test]
fn test_headers_empty_map_is_accepted_edge() {
    let id = TaskId::new();
    let payload = Bytes::from_static(b"x");
    let ack: futures::future::BoxFuture<'static, Result<(), QueueError>> =
        Box::pin(async { Ok(()) });
    let nack: futures::future::BoxFuture<'static, Result<(), QueueError>> =
        Box::pin(async { Ok(()) });
    let handle = TaskHandleBuilder::new(id, payload, ack, nack)
        .headers(HashMap::new())
        .build();
    assert!(
        handle.headers.is_empty(),
        "empty headers map must be stored as-is"
    );
}

/// @covers: TaskHandleBuilder::headers
#[test]
fn test_headers_multiple_entries_all_preserved_error() {
    let id = TaskId::new();
    let payload = Bytes::from_static(b"x");
    let mut headers = HashMap::new();
    headers.insert("a".to_owned(), "1".to_owned());
    headers.insert("b".to_owned(), "2".to_owned());
    let ack: futures::future::BoxFuture<'static, Result<(), QueueError>> =
        Box::pin(async { Ok(()) });
    let nack: futures::future::BoxFuture<'static, Result<(), QueueError>> =
        Box::pin(async { Ok(()) });
    let handle = TaskHandleBuilder::new(id, payload, ack, nack)
        .headers(headers.clone())
        .build();
    assert_eq!(
        handle.headers.len(),
        2,
        "all header entries must be preserved"
    );
}

/// @covers: TaskHandleBuilder::build
#[test]
fn test_build_produces_task_handle_with_correct_task_id_happy() {
    let id = TaskId::new();
    let payload = Bytes::from_static(b"build-test");
    let ack: futures::future::BoxFuture<'static, Result<(), QueueError>> =
        Box::pin(async { Ok(()) });
    let nack: futures::future::BoxFuture<'static, Result<(), QueueError>> =
        Box::pin(async { Ok(()) });
    let handle = TaskHandleBuilder::new(id, payload, ack, nack).build();
    assert_eq!(
        handle.task_id, id,
        "build must preserve the task_id from new()"
    );
}

/// @covers: TaskHandleBuilder::build
#[test]
fn test_build_with_large_payload_preserves_bytes_edge() {
    let id = TaskId::new();
    let large = Bytes::from(vec![0u8; 1024]);
    let ack: futures::future::BoxFuture<'static, Result<(), QueueError>> =
        Box::pin(async { Ok(()) });
    let nack: futures::future::BoxFuture<'static, Result<(), QueueError>> =
        Box::pin(async { Ok(()) });
    let handle = TaskHandleBuilder::new(id, large.clone(), ack, nack).build();
    assert_eq!(
        handle.payload, large,
        "build must preserve large payloads without truncation"
    );
}

/// @covers: TaskHandleBuilder::build
#[test]
fn test_build_ack_future_resolves_to_ok_error() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    let id = TaskId::new();
    let payload = Bytes::from_static(b"ack-test");
    let ack: futures::future::BoxFuture<'static, Result<(), QueueError>> =
        Box::pin(async { Ok(()) });
    let nack: futures::future::BoxFuture<'static, Result<(), QueueError>> =
        Box::pin(async { Ok(()) });
    let handle = TaskHandleBuilder::new(id, payload, ack, nack).build();
    let result = rt.block_on(handle.ack);
    assert!(
        result.is_ok(),
        "ack future provided to builder must resolve to Ok"
    );
}
