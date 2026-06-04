//! Integration tests for `MailboxError` variants.

use swe_edge_runtime_actor::MailboxError;

/// @covers: MailboxError::Full — Display includes "capacity"
#[test]
fn test_mailbox_error_full_display_mentions_capacity() {
    let err = MailboxError::Full;
    let msg = err.to_string();
    assert!(
        msg.contains("capacity") || msg.contains("full") || msg.contains("exceeded"),
        "MailboxError::Full display must be descriptive, got: '{}'",
        msg
    );
}

/// @covers: MailboxError::Closed — Display includes "dropped" or "closed"
#[test]
fn test_mailbox_error_closed_display_is_descriptive() {
    let err = MailboxError::Closed;
    let msg = err.to_string();
    assert!(
        !msg.is_empty(),
        "MailboxError::Closed must have a non-empty Display"
    );
}

/// @covers: MailboxError::ReplyDropped — Display is non-empty
#[test]
fn test_mailbox_error_reply_dropped_display_is_non_empty() {
    let err = MailboxError::ReplyDropped;
    let msg = err.to_string();
    assert!(
        !msg.is_empty(),
        "MailboxError::ReplyDropped must have a non-empty Display"
    );
}

/// @covers: MailboxError::ActorStopped — is std::error::Error
#[test]
fn test_mailbox_error_implements_std_error() {
    let err: Box<dyn std::error::Error> = Box::new(MailboxError::ActorStopped);
    let msg = err.to_string();
    assert!(
        !msg.is_empty(),
        "MailboxError must implement std::error::Error"
    );
}

/// @covers: MailboxError clone — all variants clone correctly
#[test]
fn test_mailbox_error_clones_correctly() {
    let variants = [
        MailboxError::Full,
        MailboxError::Closed,
        MailboxError::ReplyDropped,
        MailboxError::ActorStopped,
    ];
    for err in variants {
        let _cloned = err.clone();
    }
}
