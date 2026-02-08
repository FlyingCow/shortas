use salvo::oapi::ToSchema;
use serde::{Deserialize, Serialize};

use crate::model::Domain;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateDomainRequest {
    pub id: String,
    pub name: String,
    pub owner_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct DomainDto {
    pub id: String,
    pub name: String,
    pub owner_id: String,
    pub status: String,
    pub verification_reason: String,
    pub last_check_at: Option<i64>,
    pub next_check_at: Option<i64>,
    pub created_at: i64,
}

impl From<Domain> for DomainDto {
    fn from(domain: Domain) -> Self {
        Self {
            id: domain.id,
            name: domain.name,
            owner_id: domain.owner_id,
            status: domain.status.to_string(),
            verification_reason: domain.verification_reason.to_string(),
            last_check_at: domain.last_check_at.map(|dt| dt.timestamp_millis()),
            next_check_at: domain.next_check_at.map(|dt| dt.timestamp_millis()),
            created_at: domain.created_at.timestamp_millis(),
        }
    }
}

impl From<CreateDomainRequest> for Domain {
    fn from(req: CreateDomainRequest) -> Self {
        Domain::new(req.id, req.name.to_lowercase(), req.owner_id)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct DomainListResponse {
    pub data: Vec<DomainDto>,
    pub pagination: PaginationInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PaginationInfo {
    pub page: u32,
    pub page_size: u32,
    pub total_count: u64,
    pub total_pages: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct DnsConfigResponse {
    pub txt_record_name: String,
    pub allowed_ipv4: Vec<String>,
    pub allowed_ipv6: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ErrorResponse {
    pub error: String,
    pub code: String,
    pub message: String,
}

impl ErrorResponse {
    pub fn new(error: &str, code: &str, message: &str) -> Self {
        Self {
            error: error.to_string(),
            code: code.to_string(),
            message: message.to_string(),
        }
    }

    pub fn not_found(resource: &str, id: &str) -> Self {
        Self::new(
            "NOT_FOUND",
            "RESOURCE_NOT_FOUND",
            &format!("{} with id {} not found", resource, id),
        )
    }

    pub fn validation(field: &str, message: &str) -> Self {
        Self::new(
            "VALIDATION_ERROR",
            "INVALID_INPUT",
            &format!("{}: {}", field, message),
        )
    }

    pub fn conflict(message: &str) -> Self {
        Self::new("CONFLICT", "RESOURCE_EXISTS", message)
    }

    pub fn internal(message: &str) -> Self {
        Self::new("INTERNAL_ERROR", "INTERNAL_SERVER_ERROR", message)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{VerificationReason, VerificationStatus};
    use chrono::Utc;

    fn sample_domain() -> Domain {
        Domain {
            id: "d1".into(),
            name: "example.com".into(),
            owner_id: "owner1".into(),
            status: VerificationStatus::Verified,
            verification_reason: VerificationReason::TxtRecordValid,
            last_check_at: Some(Utc::now()),
            next_check_at: Some(Utc::now()),
            created_at: Utc::now(),
        }
    }

    #[test]
    fn test_domain_dto_from_domain() {
        let domain = sample_domain();
        let created_millis = domain.created_at.timestamp_millis();
        let last_millis = domain.last_check_at.unwrap().timestamp_millis();
        let next_millis = domain.next_check_at.unwrap().timestamp_millis();

        let dto = DomainDto::from(domain);

        assert_eq!(dto.id, "d1");
        assert_eq!(dto.name, "example.com");
        assert_eq!(dto.owner_id, "owner1");
        assert_eq!(dto.status, "verified");
        assert_eq!(dto.verification_reason, "txt_record_valid");
        assert_eq!(dto.last_check_at, Some(last_millis));
        assert_eq!(dto.next_check_at, Some(next_millis));
        assert_eq!(dto.created_at, created_millis);
    }

    #[test]
    fn test_domain_dto_from_domain_none_timestamps() {
        let domain = Domain {
            id: "d2".into(),
            name: "test.com".into(),
            owner_id: "o2".into(),
            status: VerificationStatus::Pending,
            verification_reason: VerificationReason::NotChecked,
            last_check_at: None,
            next_check_at: None,
            created_at: Utc::now(),
        };

        let dto = DomainDto::from(domain);

        assert!(dto.last_check_at.is_none());
        assert!(dto.next_check_at.is_none());
        assert_eq!(dto.status, "pending");
        assert_eq!(dto.verification_reason, "not_checked");
    }

    #[test]
    fn test_domain_dto_from_failed_domain() {
        let domain = Domain {
            id: "d3".into(),
            name: "bad.com".into(),
            owner_id: "o3".into(),
            status: VerificationStatus::Failed,
            verification_reason: VerificationReason::ARecordInvalid,
            last_check_at: Some(Utc::now()),
            next_check_at: Some(Utc::now()),
            created_at: Utc::now(),
        };

        let dto = DomainDto::from(domain);

        assert_eq!(dto.status, "failed");
        assert_eq!(dto.verification_reason, "a_record_invalid");
    }

    #[test]
    fn test_create_domain_request_to_domain() {
        let req = CreateDomainRequest {
            id: "d1".into(),
            name: "Example.COM".into(),
            owner_id: "owner1".into(),
        };

        let domain = Domain::from(req);

        assert_eq!(domain.id, "d1");
        assert_eq!(domain.name, "example.com"); // lowercased
        assert_eq!(domain.owner_id, "owner1");
        assert_eq!(domain.status, VerificationStatus::Pending);
        assert_eq!(domain.verification_reason, VerificationReason::NotChecked);
    }

    #[test]
    fn test_create_domain_request_lowercases_name() {
        let req = CreateDomainRequest {
            id: "d1".into(),
            name: "MY-DOMAIN.ORG".into(),
            owner_id: "o1".into(),
        };
        let domain = Domain::from(req);
        assert_eq!(domain.name, "my-domain.org");
    }

    #[test]
    fn test_domain_dto_serde_roundtrip() {
        let dto = DomainDto {
            id: "d1".into(),
            name: "example.com".into(),
            owner_id: "o1".into(),
            status: "verified".into(),
            verification_reason: "txt_record_valid".into(),
            last_check_at: Some(1700000000000),
            next_check_at: Some(1700001800000),
            created_at: 1699999000000,
        };

        let json = serde_json::to_string(&dto).unwrap();
        let deserialized: DomainDto = serde_json::from_str(&json).unwrap();

        assert_eq!(dto.id, deserialized.id);
        assert_eq!(dto.name, deserialized.name);
        assert_eq!(dto.status, deserialized.status);
        assert_eq!(dto.last_check_at, deserialized.last_check_at);
        assert_eq!(dto.created_at, deserialized.created_at);
    }

    #[test]
    fn test_pagination_info_serde() {
        let info = PaginationInfo {
            page: 2,
            page_size: 20,
            total_count: 55,
            total_pages: 3,
        };

        let json = serde_json::to_string(&info).unwrap();
        let deserialized: PaginationInfo = serde_json::from_str(&json).unwrap();

        assert_eq!(info.page, deserialized.page);
        assert_eq!(info.page_size, deserialized.page_size);
        assert_eq!(info.total_count, deserialized.total_count);
        assert_eq!(info.total_pages, deserialized.total_pages);
    }

    #[test]
    fn test_error_response_new() {
        let err = ErrorResponse::new("ERR", "CODE", "something went wrong");
        assert_eq!(err.error, "ERR");
        assert_eq!(err.code, "CODE");
        assert_eq!(err.message, "something went wrong");
    }

    #[test]
    fn test_error_response_not_found() {
        let err = ErrorResponse::not_found("Domain", "d123");
        assert_eq!(err.error, "NOT_FOUND");
        assert_eq!(err.code, "RESOURCE_NOT_FOUND");
        assert!(err.message.contains("Domain"));
        assert!(err.message.contains("d123"));
    }

    #[test]
    fn test_error_response_validation() {
        let err = ErrorResponse::validation("name", "cannot be empty");
        assert_eq!(err.error, "VALIDATION_ERROR");
        assert_eq!(err.code, "INVALID_INPUT");
        assert!(err.message.contains("name"));
        assert!(err.message.contains("cannot be empty"));
    }

    #[test]
    fn test_error_response_conflict() {
        let err = ErrorResponse::conflict("Domain already exists");
        assert_eq!(err.error, "CONFLICT");
        assert_eq!(err.code, "RESOURCE_EXISTS");
        assert_eq!(err.message, "Domain already exists");
    }

    #[test]
    fn test_error_response_internal() {
        let err = ErrorResponse::internal("db connection failed");
        assert_eq!(err.error, "INTERNAL_ERROR");
        assert_eq!(err.code, "INTERNAL_SERVER_ERROR");
        assert_eq!(err.message, "db connection failed");
    }

    #[test]
    fn test_error_response_serde_roundtrip() {
        let err = ErrorResponse::not_found("Domain", "abc");
        let json = serde_json::to_string(&err).unwrap();
        let deserialized: ErrorResponse = serde_json::from_str(&json).unwrap();

        assert_eq!(err.error, deserialized.error);
        assert_eq!(err.code, deserialized.code);
        assert_eq!(err.message, deserialized.message);
    }

    #[test]
    fn test_dns_config_response_serde() {
        let config = DnsConfigResponse {
            txt_record_name: "_shortas-domain-challenge".into(),
            allowed_ipv4: vec!["1.2.3.4".into()],
            allowed_ipv6: vec![],
        };

        let json = serde_json::to_string(&config).unwrap();
        let deserialized: DnsConfigResponse = serde_json::from_str(&json).unwrap();

        assert_eq!(config.txt_record_name, deserialized.txt_record_name);
        assert_eq!(config.allowed_ipv4, deserialized.allowed_ipv4);
        assert_eq!(config.allowed_ipv6, deserialized.allowed_ipv6);
    }

    #[test]
    fn test_domain_list_response_serde() {
        let resp = DomainListResponse {
            data: vec![DomainDto::from(sample_domain())],
            pagination: PaginationInfo {
                page: 1,
                page_size: 20,
                total_count: 1,
                total_pages: 1,
            },
        };

        let json = serde_json::to_string(&resp).unwrap();
        let deserialized: DomainListResponse = serde_json::from_str(&json).unwrap();

        assert_eq!(resp.data.len(), deserialized.data.len());
        assert_eq!(resp.data[0].id, deserialized.data[0].id);
        assert_eq!(resp.pagination.total_count, deserialized.pagination.total_count);
    }
}
