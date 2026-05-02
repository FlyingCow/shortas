//! User controller for user management and onboarding.

use salvo::prelude::*;

use crate::application::dto::{CompleteStepDto, OnboardingDto, UserInitDto, UserProfileDto};
use crate::domain::entities::{ApiError, UserOnboarding};
use crate::presentation::middleware::{render_error, render_success, DepotExt, UserExt};

/// Build user controller router.
pub fn user_controller() -> Router {
    Router::with_path("user")
        .push(Router::with_path("me").get(get_current_user))
        .push(Router::with_path("init").get(init_user))
        .push(Router::with_path("onboarding/complete").post(complete_onboarding_step))
}

/// Get current user profile.
#[endpoint(
    operation_id = "get_current_user",
    summary = "Get current user",
    description = "Get the current authenticated user's profile",
    tags("User"),
    responses(
        (status_code = 200, description = "User profile", body = UserProfileDto)
    )
)]
pub async fn get_current_user(depot: &mut Depot, res: &mut Response) {
    let user_profile = match depot.user_profile() {
        Ok(profile) => profile,
        Err(e) => {
            render_error(res, ApiError::unauthorized(e.to_string()));
            return;
        }
    };

    render_success(res, UserProfileDto::from_entity(user_profile.clone()));
}

/// Initialize user (create system workspace if needed).
#[endpoint(
    operation_id = "init_user",
    summary = "Initialize user",
    description = "Initialize a new user with system workspace and onboarding state",
    tags("User"),
    responses(
        (status_code = 200, description = "User initialization status", body = UserInitDto)
    )
)]
pub async fn init_user(depot: &mut Depot, res: &mut Response) {
    let app_state = match depot.app_state() {
        Ok(state) => state,
        Err(e) => {
            render_error(res, ApiError::internal(e.to_string()));
            return;
        }
    };

    let user_id = match depot.user_id() {
        Ok(id) => id,
        Err(e) => {
            render_error(res, ApiError::unauthorized(e.to_string()));
            return;
        }
    };

    // Check if user already has a system workspace
    let workspace = match app_state.workspace_repo.get_system_workspace(&user_id).await {
        Ok(Some(ws)) => ws,
        Ok(None) => {
            // Create system workspace for new user
            match app_state.workspace_repo.create_system_workspace(&user_id).await {
                Ok(ws) => ws,
                Err(e) => {
                    render_error(res, e);
                    return;
                }
            }
        }
        Err(e) => {
            render_error(res, e);
            return;
        }
    };

    // Create onboarding state
    let onboarding = UserOnboarding::new(user_id);

    render_success(
        res,
        UserInitDto {
            initialized: true,
            workspace_id: workspace.id.to_string(),
            onboarding: OnboardingDto::from_entity(onboarding),
        },
    );
}

/// Complete an onboarding step.
#[endpoint(
    operation_id = "complete_onboarding_step",
    summary = "Complete onboarding step",
    description = "Mark an onboarding step as completed",
    tags("User"),
    request_body(content = CompleteStepDto, description = "Step to complete"),
    responses(
        (status_code = 200, description = "Onboarding state", body = OnboardingDto)
    )
)]
pub async fn complete_onboarding_step(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let user_id = match depot.user_id() {
        Ok(id) => id,
        Err(e) => {
            render_error(res, ApiError::unauthorized(e.to_string()));
            return;
        }
    };

    let dto: CompleteStepDto = match req.parse_json().await {
        Ok(dto) => dto,
        Err(e) => {
            render_error(res, ApiError::validation(e.to_string()));
            return;
        }
    };

    // In a real implementation, this would persist the onboarding state
    let mut onboarding = UserOnboarding::new(user_id);
    onboarding.complete_step(&dto.step);

    render_success(res, OnboardingDto::from_entity(onboarding));
}
