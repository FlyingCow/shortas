//! Route DTOs for API requests and responses.

use salvo::oapi::ToSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

use crate::domain::entities::{
    BlockedReason, ChallengeRouting, ConditionalRouting, DestinationFormat, FileRouting,
    QrSettings, Route, RouteDomain, RouteProperties, RouteStatus, RoutingPolicy, RoutingTerminal,
};

use super::DomainDto;

/// Route response DTO.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RouteDto {
    pub id: String,
    #[serde(rename = "switch")]
    pub switch: String,
    pub link: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dest: Option<String>,
    pub dest_format: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl: Option<u64>,
    pub status: String,
    pub terminal: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<RoutingPolicyDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<RoutePropertiesDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<DomainDto>,
}

impl RouteDto {
    /// Convert from domain entity.
    pub fn from_entity(route: Route, domain: Option<RouteDomain>) -> Self {
        Self {
            id: route.id.to_string(),
            switch: route.switch,
            link: route.link,
            dest: route.dest,
            dest_format: format!("{:?}", route.dest_format),
            code: route.code,
            ttl: route.ttl,
            status: route.status.as_str().to_string(),
            terminal: format!("{:?}", route.terminal),
            policy: Some(RoutingPolicyDto::from_entity(&route.policy)),
            properties: Some(RoutePropertiesDto::from_entity(&route.properties)),
            domain_id: route.domain_id.map(|id| id.to_string()),
            domain: domain.map(DomainDto::from_entity),
        }
    }
}

/// Route creation request DTO.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateRouteDto {
    #[serde(default = "default_switch")]
    pub switch: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dest: Option<String>,
    #[serde(default = "default_dest_format")]
    pub dest_format: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl: Option<u64>,
    #[serde(default = "default_status")]
    pub status: String,
    #[serde(default = "default_terminal")]
    pub terminal: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<RoutingPolicyDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<RoutePropertiesDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_id: Option<String>,
    /// Shorthand for conditional routing (alternative to policy).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<ConditionDestinationDto>>,
}

fn default_switch() -> String {
    "main".to_string()
}

fn default_dest_format() -> String {
    "Http".to_string()
}

fn default_status() -> String {
    "Active".to_string()
}

fn default_terminal() -> String {
    "External".to_string()
}

impl CreateRouteDto {
    /// Convert to domain entity.
    pub fn to_entity(self, user_id: &str) -> Result<Route, String> {
        let domain_id_str = self.domain_id
            .ok_or("domain_id is required")?;
        let domain_id = Uuid::parse_str(&domain_id_str)
            .map_err(|_| "Invalid domain_id")?;

        let dest_format = match self.dest_format.as_str() {
            "Native" => DestinationFormat::Native,
            _ => DestinationFormat::Http,
        };

        let terminal = match self.terminal.as_str() {
            "Internal" => RoutingTerminal::Internal,
            "Middleware" => RoutingTerminal::Middleware,
            _ => RoutingTerminal::External,
        };

        let status = match self.status.as_str() {
            "Blocked" => RouteStatus::Blocked(BlockedReason::Unknown),
            _ => RouteStatus::Active,
        };

        // Build policy from conditions if provided
        let policy = if let Some(conditions) = self.conditions {
            let conds: Vec<ConditionalRouting> = conditions
                .into_iter()
                .enumerate()
                .map(|(i, c)| c.to_entity(i))
                .collect();
            RoutingPolicy::Conditional { conditions: conds }
        } else if let Some(policy_dto) = self.policy {
            policy_dto.to_entity()
        } else {
            RoutingPolicy::Basic
        };

        let mut properties = self.properties
            .map(|p| p.to_entity())
            .unwrap_or_default();

        properties.owner_id = Some(user_id.to_string());
        properties.creator_id = Some(user_id.to_string());
        properties.domain_id = Some(domain_id.to_string());

        Ok(Route {
            id: Uuid::new_v4(),
            switch: self.switch,
            link: self.link.unwrap_or_default(),
            dest: self.dest,
            dest_format,
            code: self.code,
            ttl: self.ttl,
            status,
            terminal,
            policy,
            properties,
            domain_id: Some(domain_id),
        })
    }
}

/// Route update request DTO.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdateRouteDto {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dest: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dest_format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terminal: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<RoutingPolicyDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<RoutePropertiesDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<ConditionDestinationDto>>,
}

impl UpdateRouteDto {
    /// Apply updates to existing route.
    pub fn apply_to(self, mut route: Route) -> Route {
        if let Some(dest) = self.dest {
            route.dest = Some(dest);
        }
        if let Some(dest_format) = self.dest_format {
            route.dest_format = match dest_format.as_str() {
                "Native" => DestinationFormat::Native,
                _ => DestinationFormat::Http,
            };
        }
        if let Some(code) = self.code {
            route.code = Some(code);
        }
        if let Some(ttl) = self.ttl {
            route.ttl = Some(ttl);
        }
        if let Some(status) = self.status {
            route.status = match status.as_str() {
                "Blocked" => RouteStatus::Blocked(BlockedReason::Unknown),
                _ => RouteStatus::Active,
            };
        }
        if let Some(terminal) = self.terminal {
            route.terminal = match terminal.as_str() {
                "Internal" => RoutingTerminal::Internal,
                "Middleware" => RoutingTerminal::Middleware,
                _ => RoutingTerminal::External,
            };
        }
        if let Some(conditions) = self.conditions {
            let conds: Vec<ConditionalRouting> = conditions
                .into_iter()
                .enumerate()
                .map(|(i, c)| c.to_entity(i))
                .collect();
            route.policy = RoutingPolicy::Conditional { conditions: conds };
        } else if let Some(policy) = self.policy {
            route.policy = policy.to_entity();
        }
        if let Some(props) = self.properties {
            let existing = route.properties.clone();
            route.properties = props.merge_with(existing);
        }
        route
    }
}

/// Routing policy DTO.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(tag = "type")]
pub enum RoutingPolicyDto {
    Basic,
    Conditional { conditions: Vec<ConditionalRoutingDto> },
    Challenge { challenge: Option<ChallengeRoutingDto> },
    File { file: Option<FileRoutingDto> },
    Mirroring,
}

impl RoutingPolicyDto {
    pub fn from_entity(policy: &RoutingPolicy) -> Self {
        match policy {
            RoutingPolicy::Basic => RoutingPolicyDto::Basic,
            RoutingPolicy::Conditional { conditions } => RoutingPolicyDto::Conditional {
                conditions: conditions.iter().map(ConditionalRoutingDto::from_entity).collect(),
            },
            RoutingPolicy::Challenge { challenge } => RoutingPolicyDto::Challenge {
                challenge: challenge.as_ref().map(|c| ChallengeRoutingDto {
                    key: c.key.clone(),
                    source: c.source.clone(),
                    challenge_type: c.challenge_type.clone(),
                }),
            },
            RoutingPolicy::File { file } => RoutingPolicyDto::File {
                file: file.as_ref().map(|f| FileRoutingDto {
                    content_type: f.content_type.clone(),
                }),
            },
            RoutingPolicy::Mirroring => RoutingPolicyDto::Mirroring,
            RoutingPolicy::Unknown => RoutingPolicyDto::Basic,
        }
    }

    pub fn to_entity(self) -> RoutingPolicy {
        match self {
            RoutingPolicyDto::Basic => RoutingPolicy::Basic,
            RoutingPolicyDto::Conditional { conditions } => RoutingPolicy::Conditional {
                conditions: conditions.into_iter().enumerate().map(|(i, c)| c.to_entity(i)).collect(),
            },
            RoutingPolicyDto::Challenge { challenge } => RoutingPolicy::Challenge {
                challenge: challenge.map(|c| ChallengeRouting {
                    key: c.key,
                    source: c.source,
                    challenge_type: c.challenge_type,
                }),
            },
            RoutingPolicyDto::File { file } => RoutingPolicy::File {
                file: file.map(|f| FileRouting {
                    content_type: f.content_type,
                }),
            },
            RoutingPolicyDto::Mirroring => RoutingPolicy::Mirroring,
        }
    }
}

/// Conditional routing DTO.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ConditionalRoutingDto {
    #[serde(default)]
    pub key: Option<String>,
    pub condition: Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dest: Option<String>,
}

impl ConditionalRoutingDto {
    pub fn from_entity(routing: &ConditionalRouting) -> Self {
        Self {
            key: Some(routing.key.clone()),
            condition: serde_json::to_value(&routing.condition).unwrap_or(Value::Null),
            dest: routing.dest.clone(),
        }
    }

    pub fn to_entity(self, index: usize) -> ConditionalRouting {
        ConditionalRouting {
            key: self.key.unwrap_or_else(|| format!("cond-{}", index)),
            condition: serde_json::from_value(self.condition).unwrap_or_default(),
            dest: self.dest,
        }
    }
}

/// Shorthand condition-destination for API input.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ConditionDestinationDto {
    /// Optional key for condition branch (auto-generated if not provided)
    #[serde(default)]
    pub key: Option<String>,
    pub condition: Value,
    pub dest: String,
}

impl ConditionDestinationDto {
    pub fn to_entity(self, index: usize) -> ConditionalRouting {
        ConditionalRouting {
            key: self.key.unwrap_or_else(|| format!("cond-{}", index)),
            condition: serde_json::from_value(self.condition).unwrap_or_default(),
            dest: Some(self.dest),
        }
    }
}

/// Challenge routing DTO.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ChallengeRoutingDto {
    pub key: String,
    pub source: String,
    pub challenge_type: String,
}

/// File routing DTO.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct FileRoutingDto {
    pub content_type: String,
}

/// Route properties DTO.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RoutePropertiesDto {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scripts: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub native: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundling: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qr_settings: Option<QrSettingsDto>,
    #[serde(default)]
    pub opengraph: bool,
    #[serde(default)]
    pub allow_debug: bool,
}

impl RoutePropertiesDto {
    pub fn from_entity(props: &RouteProperties) -> Self {
        Self {
            route_id: props.route_id.clone(),
            domain_id: props.domain_id.clone(),
            owner_id: props.owner_id.clone(),
            creator_id: props.creator_id.clone(),
            workspace_id: props.workspace_id.clone(),
            scripts: props.scripts.clone(),
            tags: props.tags.clone(),
            custom: props.custom.clone(),
            native: props.native.clone(),
            bundling: props.bundling.clone(),
            qr_settings: props.qr_settings.as_ref().map(QrSettingsDto::from_entity),
            opengraph: props.opengraph,
            allow_debug: props.allow_debug,
        }
    }

    pub fn to_entity(self) -> RouteProperties {
        RouteProperties {
            route_id: self.route_id,
            domain_id: self.domain_id,
            owner_id: self.owner_id,
            creator_id: self.creator_id,
            workspace_id: self.workspace_id,
            scripts: self.scripts,
            tags: self.tags,
            custom: self.custom,
            native: self.native,
            bundling: self.bundling,
            qr_settings: self.qr_settings.map(|q| q.to_entity()),
            opengraph: self.opengraph,
            allow_debug: self.allow_debug,
        }
    }

    pub fn merge_with(self, existing: RouteProperties) -> RouteProperties {
        RouteProperties {
            route_id: self.route_id.or(existing.route_id),
            domain_id: existing.domain_id, // Immutable
            owner_id: existing.owner_id,   // Immutable
            creator_id: existing.creator_id, // Immutable
            workspace_id: self.workspace_id.or(existing.workspace_id),
            scripts: self.scripts.or(existing.scripts),
            tags: self.tags.or(existing.tags),
            custom: self.custom.or(existing.custom),
            native: self.native.or(existing.native),
            bundling: self.bundling.or(existing.bundling),
            qr_settings: self.qr_settings.map(|q| q.to_entity()).or(existing.qr_settings),
            opengraph: self.opengraph,
            allow_debug: self.allow_debug,
        }
    }
}

/// QR settings DTO.
#[derive(Debug, Clone, Default, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct QrSettingsDto {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foreground_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo_size: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub margin: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_correction: Option<String>,
}

impl QrSettingsDto {
    pub fn from_entity(settings: &QrSettings) -> Self {
        Self {
            foreground_color: settings.foreground_color.clone(),
            background_color: settings.background_color.clone(),
            logo_url: settings.logo_url.clone(),
            logo_size: settings.logo_size,
            size: settings.size,
            margin: settings.margin,
            error_correction: settings.error_correction.clone(),
        }
    }

    pub fn to_entity(self) -> QrSettings {
        QrSettings {
            foreground_color: self.foreground_color,
            background_color: self.background_color,
            logo_url: self.logo_url,
            logo_size: self.logo_size,
            size: self.size,
            margin: self.margin,
            error_correction: self.error_correction,
        }
    }
}

/// Bulk operation request.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct BulkRoutesDto {
    pub routes: Vec<CreateRouteDto>,
}

/// Bulk delete request.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct BulkDeleteDto {
    pub ids: Vec<String>,
}

/// Link suggestion response.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct LinkSuggestionDto {
    pub link: String,
}

/// Presigned URL response.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PresignedUrlDto {
    pub url: String,
    pub expires_at: String,
}

/// Pagination metadata matching C# API format.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PaginationDto {
    pub page: i32,
    pub page_size: i32,
    pub total_count: i64,
    pub total_pages: i32,
}

/// Paginated routes response matching C# API format.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PaginatedRoutesDto {
    pub data: Vec<RouteDto>,
    pub pagination: PaginationDto,
}

impl PaginatedRoutesDto {
    pub fn new(routes: Vec<RouteDto>, page: i32, page_size: i32, total_count: i64) -> Self {
        let total_pages = ((total_count as f64) / (page_size as f64)).ceil() as i32;
        Self {
            data: routes,
            pagination: PaginationDto {
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

    fn minimal_create_dto(domain_id: &str) -> CreateRouteDto {
        CreateRouteDto {
            switch: "main".to_string(),
            link: Some("abc123".to_string()),
            dest: Some("https://example.com".to_string()),
            dest_format: "Http".to_string(),
            code: None,
            ttl: None,
            status: "Active".to_string(),
            terminal: "External".to_string(),
            policy: None,
            properties: None,
            domain_id: Some(domain_id.to_string()),
            conditions: None,
        }
    }

    #[test]
    fn test_create_route_dto_requires_domain_id() {
        let mut dto = minimal_create_dto(&Uuid::new_v4().to_string());
        dto.domain_id = None;
        let result = dto.to_entity("user-1");
        assert_eq!(result.unwrap_err(), "domain_id is required");
    }

    #[test]
    fn test_create_route_dto_rejects_invalid_domain_id() {
        let mut dto = minimal_create_dto("not-a-uuid");
        dto.domain_id = Some("not-a-uuid".to_string());
        let result = dto.to_entity("user-1");
        assert_eq!(result.unwrap_err(), "Invalid domain_id");
    }

    #[test]
    fn test_create_route_dto_sets_owner_creator_and_domain_on_properties() {
        let domain_id = Uuid::new_v4();
        let dto = minimal_create_dto(&domain_id.to_string());
        let route = dto.to_entity("user-1").unwrap();

        assert_eq!(route.properties.owner_id, Some("user-1".to_string()));
        assert_eq!(route.properties.creator_id, Some("user-1".to_string()));
        assert_eq!(route.properties.domain_id, Some(domain_id.to_string()));
        assert_eq!(route.domain_id, Some(domain_id));
        assert_eq!(route.policy, RoutingPolicy::Basic);
        assert_eq!(route.status, RouteStatus::Active);
        assert_eq!(route.terminal, RoutingTerminal::External);
        assert_eq!(route.dest_format, DestinationFormat::Http);
    }

    #[test]
    fn test_create_route_dto_unknown_enum_strings_fall_back_to_defaults() {
        let mut dto = minimal_create_dto(&Uuid::new_v4().to_string());
        dto.dest_format = "Bogus".to_string();
        dto.terminal = "Bogus".to_string();
        dto.status = "Bogus".to_string();
        let route = dto.to_entity("user-1").unwrap();

        assert_eq!(route.dest_format, DestinationFormat::Http);
        assert_eq!(route.terminal, RoutingTerminal::External);
        assert_eq!(route.status, RouteStatus::Active);
    }

    #[test]
    fn test_create_route_dto_conditions_shorthand_builds_conditional_policy() {
        let mut dto = minimal_create_dto(&Uuid::new_v4().to_string());
        dto.conditions = Some(vec![
            ConditionDestinationDto {
                key: Some("mobile".to_string()),
                condition: serde_json::json!({}),
                dest: "https://m.example.com".to_string(),
            },
            ConditionDestinationDto {
                key: None,
                condition: serde_json::json!({}),
                dest: "https://example.com".to_string(),
            },
        ]);

        let route = dto.to_entity("user-1").unwrap();
        match route.policy {
            RoutingPolicy::Conditional { conditions } => {
                assert_eq!(conditions.len(), 2);
                assert_eq!(conditions[0].key, "mobile");
                assert_eq!(conditions[1].key, "cond-1");
                assert_eq!(conditions[1].dest, Some("https://example.com".to_string()));
            }
            other => panic!("expected conditional policy, got {other:?}"),
        }
    }

    #[test]
    fn test_create_route_dto_status_blocked() {
        let mut dto = minimal_create_dto(&Uuid::new_v4().to_string());
        dto.status = "Blocked".to_string();
        let route = dto.to_entity("user-1").unwrap();
        assert_eq!(route.status, RouteStatus::Blocked(BlockedReason::Unknown));
    }

    fn sample_route() -> Route {
        Route {
            id: Uuid::new_v4(),
            switch: "main".to_string(),
            link: "abc123".to_string(),
            dest: Some("https://example.com".to_string()),
            dest_format: DestinationFormat::Http,
            code: None,
            ttl: None,
            status: RouteStatus::Active,
            terminal: RoutingTerminal::External,
            policy: RoutingPolicy::Basic,
            properties: RouteProperties {
                owner_id: Some("owner-1".to_string()),
                creator_id: Some("creator-1".to_string()),
                domain_id: Some("domain-1".to_string()),
                ..Default::default()
            },
            domain_id: None,
        }
    }

    #[test]
    fn test_update_route_dto_only_touches_provided_fields() {
        let route = sample_route();
        let update = UpdateRouteDto {
            dest: Some("https://new.example.com".to_string()),
            dest_format: None,
            code: None,
            ttl: None,
            status: None,
            terminal: None,
            policy: None,
            properties: None,
            conditions: None,
        };

        let updated = update.apply_to(route.clone());
        assert_eq!(updated.dest, Some("https://new.example.com".to_string()));
        // Untouched fields preserved
        assert_eq!(updated.status, route.status);
        assert_eq!(updated.terminal, route.terminal);
        assert_eq!(updated.dest_format, route.dest_format);
    }

    #[test]
    fn test_update_route_dto_conditions_override_policy() {
        let route = sample_route();
        let update = UpdateRouteDto {
            dest: None,
            dest_format: None,
            code: None,
            ttl: None,
            status: None,
            terminal: None,
            policy: Some(RoutingPolicyDto::Mirroring),
            properties: None,
            conditions: Some(vec![ConditionDestinationDto {
                key: None,
                condition: serde_json::json!({}),
                dest: "https://example.com".to_string(),
            }]),
        };

        let updated = update.apply_to(route);
        // conditions shorthand takes priority over policy when both are present
        assert!(matches!(updated.policy, RoutingPolicy::Conditional { .. }));
    }

    #[test]
    fn test_update_route_dto_properties_merge_preserves_immutable_fields() {
        let route = sample_route();
        let update = UpdateRouteDto {
            dest: None,
            dest_format: None,
            code: None,
            ttl: None,
            status: None,
            terminal: None,
            policy: None,
            properties: Some(RoutePropertiesDto {
                route_id: None,
                domain_id: Some("attempted-override".to_string()),
                owner_id: Some("attempted-override".to_string()),
                creator_id: Some("attempted-override".to_string()),
                workspace_id: Some("ws-1".to_string()),
                scripts: None,
                tags: None,
                custom: None,
                native: None,
                bundling: None,
                qr_settings: None,
                opengraph: true,
                allow_debug: false,
            }),
            conditions: None,
        };

        let updated = update.apply_to(route);
        // Immutable fields keep their original values regardless of the update DTO
        assert_eq!(updated.properties.owner_id, Some("owner-1".to_string()));
        assert_eq!(updated.properties.creator_id, Some("creator-1".to_string()));
        assert_eq!(updated.properties.domain_id, Some("domain-1".to_string()));
        // Mutable field is updated
        assert_eq!(updated.properties.workspace_id, Some("ws-1".to_string()));
        assert!(updated.properties.opengraph);
    }

    #[test]
    fn test_routing_policy_dto_roundtrip_for_each_variant() {
        assert_eq!(RoutingPolicyDto::from_entity(&RoutingPolicy::Basic).to_entity(), RoutingPolicy::Basic);
        assert_eq!(
            RoutingPolicyDto::from_entity(&RoutingPolicy::Mirroring).to_entity(),
            RoutingPolicy::Mirroring
        );

        let challenge = RoutingPolicy::Challenge {
            challenge: Some(ChallengeRouting {
                key: "k".to_string(),
                source: "recaptcha".to_string(),
                challenge_type: "v2".to_string(),
            }),
        };
        assert_eq!(RoutingPolicyDto::from_entity(&challenge).to_entity(), challenge);

        let file = RoutingPolicy::File {
            file: Some(FileRouting { content_type: "image/png".to_string() }),
        };
        assert_eq!(RoutingPolicyDto::from_entity(&file).to_entity(), file);
    }

    #[test]
    fn test_routing_policy_dto_unknown_collapses_to_basic() {
        let dto = RoutingPolicyDto::from_entity(&RoutingPolicy::Unknown);
        assert!(matches!(dto, RoutingPolicyDto::Basic));
    }

    #[test]
    fn test_conditional_routing_dto_to_entity_generates_key_when_missing() {
        let dto = ConditionalRoutingDto {
            key: None,
            condition: serde_json::json!({}),
            dest: Some("https://example.com".to_string()),
        };
        let entity = dto.to_entity(3);
        assert_eq!(entity.key, "cond-3");
    }

    #[test]
    fn test_conditional_routing_dto_to_entity_invalid_condition_falls_back_to_default() {
        let dto = ConditionalRoutingDto {
            key: Some("k".to_string()),
            condition: serde_json::json!("not-an-object"),
            dest: None,
        };
        let entity = dto.to_entity(0);
        assert_eq!(entity.condition, crate::domain::entities::Condition::default());
    }

    #[test]
    fn test_paginated_routes_dto_total_pages_rounds_up() {
        let paginated = PaginatedRoutesDto::new(vec![], 1, 10, 25);
        assert_eq!(paginated.pagination.total_pages, 3);

        let exact = PaginatedRoutesDto::new(vec![], 1, 10, 20);
        assert_eq!(exact.pagination.total_pages, 2);

        let empty = PaginatedRoutesDto::new(vec![], 1, 10, 0);
        assert_eq!(empty.pagination.total_pages, 0);
    }
}
