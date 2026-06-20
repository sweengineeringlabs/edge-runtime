//! Integration tests for the json_codec_svc SAF surface.

use swe_edge_runtime::{JsonCodec, JSON_CODEC_SVC};

/// @covers: JSON_CODEC_SVC
#[test]
fn test_json_codec_svc_slug_is_correct_happy() {
    assert_eq!(JSON_CODEC_SVC, "json_codec");
}

#[test]
fn test_json_codec_svc_slug_is_non_empty_error() {
    assert!(!JSON_CODEC_SVC.is_empty());
}

#[test]
fn test_json_codec_is_object_safe_edge() {
    fn _accept(_: &dyn JsonCodec) {}
}
