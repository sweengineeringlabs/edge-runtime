//! Integration tests for the codec_svc SAF surface.

use swe_edge_runtime::{Codec, CODEC_SVC};

/// @covers: CODEC_SVC
#[test]
fn test_codec_svc_slug_is_correct_happy() {
    assert_eq!(CODEC_SVC, "codec");
}

#[test]
fn test_codec_svc_slug_is_non_empty_error() {
    assert!(!CODEC_SVC.is_empty());
}

#[test]
fn test_codec_is_object_safe_edge() {
    fn _accept(_: &dyn Codec) {}
}
