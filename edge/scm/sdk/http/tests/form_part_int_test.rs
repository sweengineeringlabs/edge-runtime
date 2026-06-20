//! Tests for `FormPart` — multipart form upload part.
// @covers FormPart
use swe_edge_runtime_http::FormPart;

#[test]
fn test_form_part_constructs_with_all_fields_happy() {
    let part = FormPart {
        name: "file".to_string(),
        filename: Some("photo.jpg".to_string()),
        content_type: Some("image/jpeg".to_string()),
        data: b"JFIF".to_vec(),
    };
    assert_eq!(part.name, "file");
    assert_eq!(part.filename.as_deref(), Some("photo.jpg"));
    assert_eq!(part.data, b"JFIF");
}

#[test]
fn test_form_part_constructs_without_filename_error() {
    // "Error" path for a multipart part: no filename or content type (text field).
    let part = FormPart {
        name: "username".to_string(),
        filename: None,
        content_type: None,
        data: b"alice".to_vec(),
    };
    assert!(part.filename.is_none());
    assert!(part.content_type.is_none());
}

#[test]
fn test_form_part_empty_data_edge() {
    // Edge: part with empty data slice.
    let part = FormPart {
        name: "empty".to_string(),
        filename: None,
        content_type: None,
        data: vec![],
    };
    assert!(part.data.is_empty());
}
