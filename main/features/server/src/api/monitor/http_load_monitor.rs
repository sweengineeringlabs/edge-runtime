//! `HttpLoadMonitor` — HTTP inbound load-monitoring wrapper interface.

use swe_edge_ingress_http::HttpIngress;

/// Marker supertrait for HTTP inbound handlers that record load metrics.
pub trait HttpLoadMonitor: HttpIngress {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_http_load_monitor_is_object_safe() {
        fn _assert(_: &dyn HttpLoadMonitor) {}
    }
}
