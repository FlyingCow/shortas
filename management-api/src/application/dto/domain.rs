//! Domain DTOs for API requests and responses.

use salvo::oapi::ToSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::entities::RouteDomain;

/// Domain response DTO.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct DomainDto {
    pub id: String,
    pub name: String,
    pub owner_id: String,
    pub is_shared: bool,
    pub verification_status: String,
    pub verification_reason: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_verification_check: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_index_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_not_found_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
}

impl DomainDto {
    pub fn from_entity(domain: RouteDomain) -> Self {
        Self {
            id: domain.id.to_string(),
            name: domain.name,
            owner_id: domain.owner_id,
            is_shared: domain.is_shared,
            verification_status: domain.verification_status.as_str().to_string(),
            verification_reason: domain.verification_reason,
            last_verification_check: domain.last_verification_check.map(|dt| dt.to_rfc3339()),
            custom_index_url: domain.custom_index_url,
            custom_not_found_url: domain.custom_not_found_url,
            created_at: domain.created_at.map(|dt| dt.to_rfc3339()),
        }
    }
}

/// Domain creation request DTO.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateDomainDto {
    pub name: String,
    #[serde(default)]
    pub is_shared: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_index_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_not_found_url: Option<String>,
}

impl CreateDomainDto {
    pub fn to_entity(self, owner_id: &str) -> RouteDomain {
        let mut domain = RouteDomain::new(self.name, owner_id.to_string());
        domain.is_shared = self.is_shared;
        domain.custom_index_url = self.custom_index_url;
        domain.custom_not_found_url = self.custom_not_found_url;
        domain
    }
}

/// Domain update request DTO.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UpdateDomainDto {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_shared: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_index_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_not_found_url: Option<String>,
}

impl UpdateDomainDto {
    pub fn apply_to(self, mut domain: RouteDomain) -> RouteDomain {
        if let Some(is_shared) = self.is_shared {
            domain.is_shared = is_shared;
        }
        if let Some(url) = self.custom_index_url {
            domain.custom_index_url = Some(url);
        }
        if let Some(url) = self.custom_not_found_url {
            domain.custom_not_found_url = Some(url);
        }
        domain
    }
}

/// DNS configuration response DTO.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct DnsConfigDto {
    pub txt_record: String,
    pub cname_target: String,
    pub a_records: Vec<String>,
}

/// Domain verification request.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct VerifyDomainDto {
    pub domain_id: String,
}

impl VerifyDomainDto {
    pub fn parse_id(&self) -> Result<Uuid, String> {
        Uuid::parse_str(&self.domain_id).map_err(|_| "Invalid domain_id".to_string())
    }
}
