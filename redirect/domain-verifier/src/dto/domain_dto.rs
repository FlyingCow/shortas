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
