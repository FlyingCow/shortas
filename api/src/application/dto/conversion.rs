//! Conversion DTOs for API requests and responses.

use rust_decimal::Decimal;
use salvo::oapi::ToSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::entities::{
    Conversion, ConversionDefinition, ConversionStatus, ConversionStep, CountingMode,
    RouteDomain,
};

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ConversionStepDto {
    /// Identifies the step in events. Cannot be renamed.
    pub key: String,
    pub name: String,
    /// Decimal string, used when an event has no value.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[salvo(schema(value_type = Option<String>))]
    pub default_value: Option<Decimal>,
}

impl From<ConversionStepDto> for ConversionStep {
    fn from(dto: ConversionStepDto) -> Self {
        Self {
            key: dto.key,
            name: dto.name,
            default_value: dto.default_value,
            archived: false,
        }
    }
}

impl From<ConversionStep> for ConversionStepDto {
    fn from(step: ConversionStep) -> Self {
        Self {
            key: step.key,
            name: step.name,
            default_value: step.default_value.map(|v| v.normalize()),
        }
    }
}

/// Create or replace a conversion's definition.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ConversionInputDto {
    pub name: String,
    pub currency: String,
    pub window_days: i32,
    pub counting: CountingMode,
    #[salvo(schema(value_type = Option<String>))]
    pub domain_id: Option<Uuid>,
    #[serde(default)]
    pub allowed_origins: Vec<String>,
    #[serde(default)]
    pub server_values_only: bool,
    /// Active steps in order. The last one is the conversion.
    pub steps: Vec<ConversionStepDto>,
    /// Routes to cover. Leave out to cover the whole workspace.
    #[salvo(schema(value_type = Option<Vec<String>>))]
    pub route_ids: Option<Vec<Uuid>>,
}

impl From<ConversionInputDto> for ConversionDefinition {
    fn from(dto: ConversionInputDto) -> Self {
        Self {
            name: dto.name.trim().to_string(),
            currency: dto.currency,
            window_days: dto.window_days,
            counting: dto.counting,
            domain_id: dto.domain_id,
            allowed_origins: dto.allowed_origins,
            server_values_only: dto.server_values_only,
            steps: dto.steps.into_iter().map(Into::into).collect(),
            route_filter: dto.route_ids,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdateConversionDto {
    /// The version being edited. A newer stored version is a conflict.
    pub version: i64,
    #[serde(flatten)]
    pub definition: ConversionInputDto,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ConversionDto {
    pub id: String,
    pub workspace_id: String,
    pub status: ConversionStatus,
    pub version: i64,
    pub public_key: String,
    pub archived_steps: Vec<ConversionStepDto>,
    pub created_at: String,
    pub updated_at: String,
    #[serde(flatten)]
    pub definition: ConversionInputDto,
}

impl From<Conversion> for ConversionDto {
    fn from(c: Conversion) -> Self {
        let d = c.definition;
        Self {
            id: c.id.to_string(),
            workspace_id: c.workspace_id.to_string(),
            status: c.status,
            version: c.version,
            public_key: c.public_key,
            archived_steps: c.archived_steps.into_iter().map(Into::into).collect(),
            created_at: c.created_at.to_rfc3339(),
            updated_at: c.updated_at.to_rfc3339(),
            definition: ConversionInputDto {
                name: d.name,
                currency: d.currency,
                window_days: d.window_days,
                counting: d.counting,
                domain_id: d.domain_id,
                allowed_origins: d.allowed_origins,
                server_values_only: d.server_values_only,
                steps: d.steps.into_iter().map(Into::into).collect(),
                route_ids: d.route_filter,
            },
        }
    }
}

/// A domain a workspace's conversions use or can use.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ConversionDomainDto {
    pub id: String,
    pub name: String,
    pub is_shared: bool,
    pub verified: bool,
}

impl From<RouteDomain> for ConversionDomainDto {
    fn from(d: RouteDomain) -> Self {
        Self {
            verified: d.is_verified(),
            id: d.id.to_string(),
            name: d.name,
            is_shared: d.is_shared,
        }
    }
}

/// Returned only when a secret key is created or rotated.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ConversionWithSecretDto {
    pub secret_key: String,
    #[serde(flatten)]
    pub conversion: ConversionDto,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::entities::conversion::tests::definition;

    #[test]
    fn responses_never_contain_the_secret_or_its_hash() {
        let (c, secret) = Conversion::new(Uuid::new_v4(), definition(), "u".into());
        let hash = c.secret_hash.clone();
        let json = serde_json::to_string(&ConversionDto::from(c)).unwrap();
        assert!(!json.contains(&secret) && !json.contains(&hash));
    }

    #[test]
    fn update_parses_flat_json() {
        let dto: UpdateConversionDto = serde_json::from_value(serde_json::json!({
            "version": 3, "name": " Sale ", "currency": "EUR", "windowDays": 30,
            "counting": "Every",
            "steps": [{ "key": "sale", "name": "Sale", "defaultValue": "5.25" }]
        }))
        .unwrap();
        assert_eq!(dto.version, 3);
        let d = ConversionDefinition::from(dto.definition);
        assert_eq!(d.name, "Sale");
        assert_eq!(d.route_filter, None);
        assert_eq!(d.steps[0].default_value.unwrap().to_string(), "5.25");
    }
}
