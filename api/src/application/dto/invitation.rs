//! Workspace invitation DTOs for API requests and responses.

use salvo::oapi::ToSchema;
use serde::{Deserialize, Serialize};

use crate::domain::entities::WorkspaceInvitation;

/// Create invitation request DTO.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateInvitationDto {
    pub email: String,
    #[serde(default = "default_invite_role")]
    pub role: String,
}

fn default_invite_role() -> String {
    "Member".to_string()
}

/// Invitation response DTO (for the workspace admin's "pending invites" list).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct InvitationDto {
    pub id: String,
    pub workspace_id: String,
    pub invited_email: String,
    pub role: String,
    pub invited_by: String,
    pub status: String,
    pub expires_at: String,
    pub created_at: String,
}

impl InvitationDto {
    pub fn from_entity(invitation: WorkspaceInvitation) -> Self {
        Self {
            id: invitation.id.to_string(),
            workspace_id: invitation.workspace_id.to_string(),
            invited_email: invitation.invited_email,
            role: invitation.role,
            invited_by: invitation.invited_by,
            status: invitation.status.as_str().to_string(),
            expires_at: invitation.expires_at.to_rfc3339(),
            created_at: invitation.created_at.to_rfc3339(),
        }
    }
}

/// Public invitation preview DTO, returned by the unauthenticated
/// "look up by token" endpoint used by the email link landing page.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct InvitationPreviewDto {
    pub workspace_name: String,
    pub inviter_name: String,
    pub role: String,
    pub invited_email: String,
    pub status: String,
}
