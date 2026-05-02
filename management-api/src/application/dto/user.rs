//! User DTOs for API requests and responses.

use salvo::oapi::ToSchema;
use serde::{Deserialize, Serialize};

use crate::domain::entities::{UserOnboarding, UserProfile};

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
