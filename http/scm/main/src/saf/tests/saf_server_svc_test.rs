//! Tests verifying that SAF constants and re-exports are accessible and correct.

#[cfg(test)]
mod tests {
    use crate::saf::{HTTP_SERVER_SVC, HTTP_SERVER_SVC_FACTORY, TLS_SVC};

    /// @covers: HTTP_SERVER_SVC
    #[test]
    fn test_http_server_svc_slug_is_non_empty_happy() {
        assert!(
            !HTTP_SERVER_SVC.is_empty(),
            "HTTP_SERVER_SVC slug must be non-empty"
        );
    }

    /// @covers: HTTP_SERVER_SVC_FACTORY
    #[test]
    fn test_http_server_svc_factory_slug_is_non_empty_happy() {
        assert!(
            !HTTP_SERVER_SVC_FACTORY.is_empty(),
            "HTTP_SERVER_SVC_FACTORY slug must be non-empty"
        );
    }

    /// @covers: TLS_SVC
    #[test]
    fn test_tls_svc_slug_is_non_empty_happy() {
        assert!(!TLS_SVC.is_empty(), "TLS_SVC slug must be non-empty");
    }

    /// @covers: HTTP_SERVER_SVC
    #[test]
    fn test_http_server_svc_slug_contains_no_whitespace_edge() {
        assert!(
            !HTTP_SERVER_SVC.contains(char::is_whitespace),
            "HTTP_SERVER_SVC slug must not contain whitespace"
        );
    }

    /// @covers: TLS_SVC
    #[test]
    fn test_tls_svc_slug_is_distinct_from_http_server_svc_edge() {
        assert_ne!(
            TLS_SVC, HTTP_SERVER_SVC,
            "TLS_SVC and HTTP_SERVER_SVC must be distinct slugs"
        );
    }
}
