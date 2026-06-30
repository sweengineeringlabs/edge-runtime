//! External tests for the grpc SAF service-marker re-exports, including the
//! `TLS_SVC` marker now sourced from edge-security-runtime.

#[cfg(test)]
mod tests {
    use crate::saf::{GRPC_SERVER_SVC, TLS_SVC};

    /// @covers: GRPC_SERVER_SVC
    #[test]
    fn test_grpc_server_svc_slug_is_non_empty_happy() {
        assert!(
            !GRPC_SERVER_SVC.is_empty(),
            "GRPC_SERVER_SVC slug must be non-empty"
        );
    }

    /// @covers: GRPC_SERVER_SVC
    #[test]
    fn test_grpc_server_svc_slug_has_no_whitespace_edge() {
        assert!(
            !GRPC_SERVER_SVC.contains(char::is_whitespace),
            "GRPC_SERVER_SVC slug must not contain whitespace"
        );
    }

    /// @covers: TLS_SVC
    #[test]
    fn test_tls_svc_reexport_value_is_tls_happy() {
        // The TLS_SVC marker is re-exported from edge-security-runtime; its value
        // is part of the stable wiring contract consumers key off.
        assert_eq!(
            TLS_SVC, "tls",
            "re-exported TLS_SVC marker must equal \"tls\""
        );
    }

    /// @covers: TLS_SVC
    #[test]
    fn test_tls_svc_distinct_from_grpc_server_svc_edge() {
        assert_ne!(
            TLS_SVC, GRPC_SERVER_SVC,
            "TLS_SVC and GRPC_SERVER_SVC must be distinct service markers"
        );
    }
}
