//! Workspace DTOs for API requests and responses.

use salvo::oapi::ToSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::entities::{ApiError, UserWorkspace, Workspace};
use crate::domain::traits::{RouteInfo, WorkspaceStats};

/// Matches the `workspaces.name VARCHAR(255)` column - caught here with a
/// clear message instead of as a raw truncation error from Postgres.
const WORKSPACE_NAME_MAX_LEN: usize = 255;
/// `description` is unbounded TEXT in the database, but still capped for
/// sane UI rendering and to avoid unbounded payloads.
const WORKSPACE_DESCRIPTION_MAX_LEN: usize = 2000;

/// Workspace response DTO.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceDto {
    pub id: String,
    pub name: String,
    pub description: String,
    pub workspace_type: String,
    pub is_system: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_count: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_count: Option<i64>,
    /// The current caller's role in this workspace ("Owner", "Admin", or
    /// "Member") - lets the UI group "my workspaces" vs "shared with me"
    /// without an extra per-workspace members fetch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_role: Option<String>,
}

impl WorkspaceDto {
    pub fn from_entity(workspace: Workspace) -> Self {
        let is_system = workspace.is_system();
        Self {
            id: workspace.id.to_string(),
            name: workspace.name,
            description: workspace.description,
            workspace_type: workspace.workspace_type,
            is_system,
            created_at: workspace.created_at.map(|dt| dt.to_rfc3339()),
            updated_at: workspace.updated_at.map(|dt| dt.to_rfc3339()),
            member_count: None,
            route_count: None,
            user_role: None,
        }
    }

    pub fn with_counts(mut self, member_count: i32, route_count: i64) -> Self {
        self.member_count = Some(member_count);
        self.route_count = Some(route_count);
        self
    }

    pub fn with_user_role(mut self, role: Option<String>) -> Self {
        self.user_role = role;
        self
    }
}

/// Lightweight workspace counters, maintained incrementally instead of
/// derived by fetching and scanning the workspace's full route list.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceStatsDto {
    pub workspace_id: String,
    pub route_count: i64,
    pub active_route_count: i64,
    pub blocked_route_count: i64,
    pub member_count: i64,
    /// A short sample of currently-blocked routes (not the full list) for a
    /// "needs attention" UI - use `blocked_route_count` for the true total.
    pub blocked_routes: Vec<BlockedRouteSummaryDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

impl WorkspaceStatsDto {
    pub fn from_stats(stats: WorkspaceStats, blocked_routes: Vec<RouteInfo>) -> Self {
        Self {
            workspace_id: stats.workspace_id.to_string(),
            route_count: stats.route_count,
            active_route_count: stats.active_route_count,
            blocked_route_count: stats.blocked_route_count,
            member_count: stats.member_count,
            blocked_routes: blocked_routes.into_iter().map(BlockedRouteSummaryDto::from_info).collect(),
            updated_at: stats.updated_at.map(|dt| dt.to_rfc3339()),
        }
    }
}

/// A minimal route reference, just enough for a dashboard link.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct BlockedRouteSummaryDto {
    pub id: String,
    pub link: String,
}

impl BlockedRouteSummaryDto {
    pub fn from_info(info: RouteInfo) -> Self {
        Self {
            id: info.id.to_string(),
            link: info.link,
        }
    }
}

/// Workspace creation request DTO.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateWorkspaceDto {
    pub name: String,
    #[serde(default)]
    pub description: String,
}

impl CreateWorkspaceDto {
    pub fn validate(&self) -> Result<(), ApiError> {
        validate_name(&self.name)?;
        validate_description(&self.description)?;
        Ok(())
    }

    pub fn to_entity(self) -> Workspace {
        let mut workspace = Workspace::new(self.name, "User".to_string());
        workspace.description = self.description;
        workspace
    }
}

fn validate_name(name: &str) -> Result<(), ApiError> {
    if name.trim().is_empty() {
        return Err(ApiError::required("name"));
    }
    if name.len() > WORKSPACE_NAME_MAX_LEN {
        return Err(ApiError::validation(format!(
            "Name must be {} characters or fewer",
            WORKSPACE_NAME_MAX_LEN
        )));
    }
    Ok(())
}

fn validate_description(description: &str) -> Result<(), ApiError> {
    if description.len() > WORKSPACE_DESCRIPTION_MAX_LEN {
        return Err(ApiError::validation(format!(
            "Description must be {} characters or fewer",
            WORKSPACE_DESCRIPTION_MAX_LEN
        )));
    }
    Ok(())
}

/// Workspace update request DTO.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UpdateWorkspaceDto {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl UpdateWorkspaceDto {
    pub fn validate(&self) -> Result<(), ApiError> {
        if let Some(name) = &self.name {
            validate_name(name)?;
        }
        if let Some(description) = &self.description {
            validate_description(description)?;
        }
        Ok(())
    }

    pub fn apply_to(self, mut workspace: Workspace) -> Workspace {
        if let Some(name) = self.name {
            workspace.name = name;
        }
        if let Some(description) = self.description {
            workspace.description = description;
        }
        workspace
    }
}

/// Workspace member DTO.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceMemberDto {
    pub id: String,
    pub user_id: String,
    pub workspace_id: String,
    pub role: String,
    pub is_owner: bool,
    pub is_admin: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub joined_at: Option<String>,
    /// The member's email, resolved from the local user directory - falls
    /// back to the raw user ID client-side if this user was never seen.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl WorkspaceMemberDto {
    pub fn from_entity(membership: UserWorkspace) -> Self {
        Self {
            id: membership.id.to_string(),
            user_id: membership.user_id.clone(),
            workspace_id: membership.workspace_id.to_string(),
            role: membership.role.clone(),
            is_owner: membership.is_owner(),
            is_admin: membership.is_admin(),
            joined_at: membership.joined_at.map(|dt| dt.to_rfc3339()),
            email: None,
            name: None,
        }
    }

    pub fn with_user_info(mut self, email: Option<String>, name: Option<String>) -> Self {
        self.email = email;
        self.name = name;
        self
    }
}

/// Add member request DTO.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AddMemberDto {
    pub user_id: String,
    #[serde(default = "default_role")]
    pub role: String,
}

fn default_role() -> String {
    "Member".to_string()
}

impl AddMemberDto {
    pub fn to_entity(self, workspace_id: Uuid) -> UserWorkspace {
        UserWorkspace::new(self.user_id, workspace_id, self.role)
    }
}

/// Update member role request DTO.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UpdateMemberRoleDto {
    pub role: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::entities::ErrorCode;

    #[test]
    fn test_create_workspace_dto_validate_rejects_empty_name() {
        let dto = CreateWorkspaceDto { name: "   ".to_string(), description: String::new() };
        let err = dto.validate().unwrap_err();
        assert_eq!(err.code, ErrorCode::ValidationError);
    }

    #[test]
    fn test_create_workspace_dto_validate_rejects_name_too_long() {
        let dto = CreateWorkspaceDto {
            name: "a".repeat(WORKSPACE_NAME_MAX_LEN + 1),
            description: String::new(),
        };
        assert!(dto.validate().is_err());
    }

    #[test]
    fn test_create_workspace_dto_validate_rejects_description_too_long() {
        let dto = CreateWorkspaceDto {
            name: "Acme".to_string(),
            description: "a".repeat(WORKSPACE_DESCRIPTION_MAX_LEN + 1),
        };
        assert!(dto.validate().is_err());
    }

    #[test]
    fn test_create_workspace_dto_validate_accepts_valid_input() {
        let dto = CreateWorkspaceDto { name: "Acme".to_string(), description: "desc".to_string() };
        assert!(dto.validate().is_ok());
    }

    #[test]
    fn test_create_workspace_dto_to_entity_sets_user_type_and_description() {
        let dto = CreateWorkspaceDto { name: "Acme".to_string(), description: "desc".to_string() };
        let workspace = dto.to_entity();
        assert_eq!(workspace.name, "Acme");
        assert_eq!(workspace.workspace_type, "User");
        assert_eq!(workspace.description, "desc");
        assert!(!workspace.is_system());
    }

    #[test]
    fn test_update_workspace_dto_validate_only_checks_provided_fields() {
        let dto = UpdateWorkspaceDto { name: None, description: None };
        assert!(dto.validate().is_ok());

        let dto = UpdateWorkspaceDto { name: Some("   ".to_string()), description: None };
        assert!(dto.validate().is_err());
    }

    #[test]
    fn test_update_workspace_dto_apply_to_only_touches_provided_fields() {
        let workspace = Workspace::new("Original".to_string(), "User".to_string());
        let dto = UpdateWorkspaceDto { name: Some("Renamed".to_string()), description: None };
        let updated = dto.apply_to(workspace.clone());
        assert_eq!(updated.name, "Renamed");
        assert_eq!(updated.description, workspace.description);
    }

    #[test]
    fn test_workspace_dto_from_entity_and_builders() {
        let workspace = Workspace::new("Acme".to_string(), "System".to_string());
        let dto = WorkspaceDto::from_entity(workspace)
            .with_counts(5, 42)
            .with_user_role(Some("Owner".to_string()));

        assert!(dto.is_system);
        assert_eq!(dto.member_count, Some(5));
        assert_eq!(dto.route_count, Some(42));
        assert_eq!(dto.user_role, Some("Owner".to_string()));
    }

    #[test]
    fn test_add_member_dto_to_entity_builds_user_workspace() {
        let workspace_id = Uuid::new_v4();
        let dto = AddMemberDto { user_id: "user-1".to_string(), role: "Admin".to_string() };
        let membership = dto.to_entity(workspace_id);
        assert_eq!(membership.user_id, "user-1");
        assert_eq!(membership.workspace_id, workspace_id);
        assert!(membership.is_admin());
        assert!(!membership.is_owner());
    }

    #[test]
    fn test_workspace_member_dto_from_entity_reflects_role_flags() {
        let membership = UserWorkspace::new("user-1".to_string(), Uuid::new_v4(), "Owner".to_string());
        let dto = WorkspaceMemberDto::from_entity(membership).with_user_info(
            Some("user@example.com".to_string()),
            Some("User One".to_string()),
        );
        assert!(dto.is_owner);
        assert!(dto.is_admin);
        assert_eq!(dto.email, Some("user@example.com".to_string()));
        assert_eq!(dto.name, Some("User One".to_string()));
    }
}
