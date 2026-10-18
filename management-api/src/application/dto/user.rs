//! User DTOs for API requests and responses.

use salvo::oapi::ToSchema;
use serde::{Deserialize, Serialize};

use crate::domain::entities::{UserOnboarding, UserProfile, Workspace};

/// User profile response DTO.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UserProfileDto {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_username: Option<String>,
    pub email_verified: bool,
    pub is_admin: bool,
}

impl UserProfileDto {
    pub fn from_entity(profile: UserProfile) -> Self {
        let is_admin = profile.is_admin();
        Self {
            id: profile.id,
            email: profile.email,
            name: profile.name,
            preferred_username: profile.preferred_username,
            email_verified: profile.email_verified,
            is_admin,
        }
    }
}

/// Workspace DTO for initialization response.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct InitWorkspaceDto {
    pub id: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "type")]
    pub workspace_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_role: Option<String>,
}

impl InitWorkspaceDto {
    pub fn from_entity(workspace: &Workspace, role: Option<String>) -> Self {
        Self {
            id: workspace.id.to_string(),
            name: workspace.name.clone(),
            description: if workspace.description.is_empty() {
                None
            } else {
                Some(workspace.description.clone())
            },
            workspace_type: workspace.workspace_type.clone(),
            created_at: workspace.created_at.map(|dt| dt.to_rfc3339()),
            updated_at: workspace.updated_at.map(|dt| dt.to_rfc3339()),
            user_role: role,
        }
    }
}

/// User settings DTO for initialization response.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserSettingsDto {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    pub status: String,
    pub debug: bool,
    pub overflow: bool,
    pub skip_tracking: Vec<String>,
    pub allowed_request_params: Vec<String>,
    pub allowed_destination_params: Vec<String>,
}

impl Default for UserSettingsDto {
    fn default() -> Self {
        Self {
            email: None,
            status: "Active".to_string(),
            debug: false,
            overflow: false,
            skip_tracking: Vec::new(),
            allowed_request_params: Vec::new(),
            allowed_destination_params: Vec::new(),
        }
    }
}

/// Initialization response DTO (matches C# InitializationResponse).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct InitializationResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace: Option<InitWorkspaceDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_settings: Option<UserSettingsDto>,
    pub message: String,
}

/// Initialization status response DTO (matches C# InitializationStatusResponse).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct InitializationStatusResponse {
    pub needs_initialization: bool,
    pub has_workspaces: bool,
    pub has_domains: bool,
    pub has_user_settings: bool,
}

/// User initialization response DTO.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UserInitDto {
    pub initialized: bool,
    pub workspace_id: String,
    pub onboarding: OnboardingDto,
}

/// Onboarding state DTO.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct OnboardingDto {
    pub completed: bool,
    pub current_step: i32,
    pub steps_completed: Vec<String>,
}

impl OnboardingDto {
    pub fn from_entity(onboarding: UserOnboarding) -> Self {
        Self {
            completed: onboarding.completed,
            current_step: onboarding.current_step,
            steps_completed: onboarding.steps_completed,
        }
    }
}

/// Complete onboarding step request DTO.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CompleteStepDto {
    pub step: String,
}
