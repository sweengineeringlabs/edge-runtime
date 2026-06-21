//! gRPC principal for authenticated request contexts.

use edge_domain::Principal;

pub(super) struct GrpcPrincipal {
    pub(super) sub: String,
}

impl Principal for GrpcPrincipal {
    fn id(&self) -> &str {
        &self.sub
    }

    fn kind(&self) -> &str {
        const KIND: &str = "grpc";
        KIND
    }
}

#[cfg(test)]
mod tests {
    use super::GrpcPrincipal;
    use edge_domain::Principal;

    #[test]
    fn test_grpc_principal_id_returns_sub() {
        let p = GrpcPrincipal {
            sub: "user-123".to_string(),
        };
        assert_eq!(p.id(), "user-123");
    }

    #[test]
    fn test_grpc_principal_kind_returns_grpc() {
        let p = GrpcPrincipal { sub: String::new() };
        assert_eq!(p.kind(), "grpc");
    }
}
