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
