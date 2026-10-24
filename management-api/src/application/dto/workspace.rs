//! Workspace DTOs for API requests and responses.

use salvo::oapi::ToSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::entities::{UserWorkspace, Workspace};

/// Workspace response DTO.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
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
        }
    }

    pub fn with_counts(mut self, member_count: i32, route_count: i64) -> Self {
        self.member_count = Some(member_count);
        self.route_count = Some(route_count);
        self
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
    pub fn to_entity(self) -> Workspace {
        let mut workspace = Workspace::new(self.name, "User".to_string());
        workspace.description = self.description;
        workspace
    }
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
pub struct WorkspaceMemberDto {
    pub id: String,
    pub user_id: String,
    pub workspace_id: String,
    pub role: String,
    pub is_owner: bool,
    pub is_admin: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub joined_at: Option<String>,
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
        }
    }
}

/// Add member request DTO.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
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
