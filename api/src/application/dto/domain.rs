//! Domain DTOs for API requests and responses.

use salvo::oapi::ToSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::entities::RouteDomain;

/// Domain response DTO.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
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
    pub next_verification_check: Option<String>,
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
            next_verification_check: domain.next_verification_check.map(|dt| dt.to_rfc3339()),
            custom_index_url: domain.custom_index_url,
            custom_not_found_url: domain.custom_not_found_url,
            created_at: domain.created_at.map(|dt| dt.to_rfc3339()),
        }
    }
}

/// A minimal domain reference, just enough for a dashboard link.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct DomainSummaryDto {
    pub id: String,
    pub name: String,
    pub verification_status: String,
}

impl DomainSummaryDto {
    pub fn from_entity(domain: RouteDomain) -> Self {
        Self {
            id: domain.id.to_string(),
            name: domain.name,
            verification_status: domain.verification_status.as_str().to_string(),
        }
    }
}

/// Count and short sample of a user's not-yet-verified domains.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct DomainAttentionDto {
    pub unverified_count: i64,
    pub domains: Vec<DomainSummaryDto>,
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

/// Custom index/404 page response DTO, keyed by domain name rather than ID
/// to match the dashboard's Custom Pages dialog, which only has the domain
/// name on hand.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CustomPagesDto {
    pub domain_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_index_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_not_found_url: Option<String>,
}

impl CustomPagesDto {
    pub fn from_entity(domain: &RouteDomain) -> Self {
        Self {
            domain_name: domain.name.clone(),
            custom_index_url: domain.custom_index_url.clone(),
            custom_not_found_url: domain.custom_not_found_url.clone(),
        }
    }
}

/// Custom index/404 page update request. The dashboard always sends both
/// fields on every save (a string, or `null` to use the default page), so
/// unlike `UpdateDomainDto` this is a full replace of both fields rather
/// than a merge-patch — no need to distinguish "omitted" from "null".
#[derive(Debug, Clone, Default, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdateCustomPagesDto {
    #[serde(default)]
    pub custom_index_url: Option<String>,
    #[serde(default)]
    pub custom_not_found_url: Option<String>,
}

impl UpdateCustomPagesDto {
    pub fn apply_to(self, mut domain: RouteDomain) -> RouteDomain {
        domain.custom_index_url = self.custom_index_url;
        domain.custom_not_found_url = self.custom_not_found_url;
        domain
    }
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

/// Pagination metadata matching C# API format.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct DomainPaginationDto {
    pub page: i32,
    pub page_size: i32,
    pub total_count: i64,
    pub total_pages: i32,
}

/// Paginated domains response matching C# API format.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PaginatedDomainsDto {
    pub data: Vec<DomainDto>,
    pub pagination: DomainPaginationDto,
}

impl PaginatedDomainsDto {
    pub fn new(domains: Vec<DomainDto>, page: i32, page_size: i32, total_count: i64) -> Self {
        let total_pages = ((total_count as f64) / (page_size as f64)).ceil() as i32;
        Self {
            data: domains,
            pagination: DomainPaginationDto {
                page,
                page_size,
                total_count,
                total_pages,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_custom_pages_dto_deserializes_camel_case_body() {
        // The dashboard sends camelCase JSON (customIndexUrl/customNotFoundUrl).
        // Without #[serde(rename_all = "camelCase")] these silently fail to
        // match the snake_case field names and fall back to their #[serde(default)]
        // of None, so every save would clear both URLs instead of setting them.
        let dto: UpdateCustomPagesDto = serde_json::from_str(
            r#"{"customIndexUrl":"https://example.com/welcome","customNotFoundUrl":null}"#,
        )
        .unwrap();
        assert_eq!(dto.custom_index_url, Some("https://example.com/welcome".to_string()));
        assert_eq!(dto.custom_not_found_url, None);
    }

    #[test]
    fn test_update_custom_pages_dto_apply_to_sets_both_fields() {
        let domain = RouteDomain::new("example.com".to_string(), "owner-1".to_string());
        let dto = UpdateCustomPagesDto {
            custom_index_url: Some("https://example.com/welcome".to_string()),
            custom_not_found_url: Some("https://example.com/404".to_string()),
        };
        let updated = dto.apply_to(domain);
        assert_eq!(updated.custom_index_url, Some("https://example.com/welcome".to_string()));
        assert_eq!(updated.custom_not_found_url, Some("https://example.com/404".to_string()));
    }
}
