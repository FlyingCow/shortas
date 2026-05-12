//! User controller for user management and onboarding.

use salvo::prelude::*;

use crate::application::dto::{
    CompleteStepDto, InitWorkspaceDto, InitializationResponse, InitializationStatusResponse,
    OnboardingDto, UserProfileDto, UserSettingsDto,
};
use crate::domain::entities::{ApiError, UserOnboarding};
use crate::presentation::middleware::{render_error, render_success, DepotExt, UserExt};

use super::cors_preflight;

/// Build user controller router.
pub fn user_controller() -> Router {
    Router::with_path("user")
        .options(cors_preflight)
        .push(Router::with_path("me").get(get_current_user).options(cors_preflight))
        .push(Router::with_path("initialize").post(initialize_user).options(cors_preflight))
        .push(Router::with_path("initialization-status").get(get_initialization_status).options(cors_preflight))
        .push(Router::with_path("onboarding/complete").post(complete_onboarding_step).options(cors_preflight))
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

/// Initialize user (create system workspace and user settings if needed).
#[endpoint(
    operation_id = "initialize_user",
    summary = "Initialize user",
    description = "Initialize default workspace and user settings for a new user",
    tags("User"),
    responses(
        (status_code = 200, description = "Initialization result with workspace and user settings", body = InitializationResponse)
    )
)]
pub async fn initialize_user(depot: &mut Depot, res: &mut Response) {
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

    let user_email = depot.user_email().ok();

    // Check if user already has workspaces
    let workspaces = match app_state.workspace_repo.list_by_user(&user_id).await {
        Ok(ws) => ws,
        Err(e) => {
            render_error(res, e);
            return;
        }
    };

    let workspace = if !workspaces.is_empty() {
        // User already has a workspace
        tracing::info!("User {} already has workspace {}", user_id, workspaces[0].id);
        workspaces.into_iter().next().unwrap()
    } else {
        // Create default workspace as System type
        match app_state.workspace_repo.create_system_workspace(&user_id).await {
            Ok(ws) => {
                tracing::info!("Created default workspace {} for user {}", ws.id, user_id);
                ws
            }
            Err(e) => {
                tracing::error!("Failed to create workspace for user {}: {:?}", user_id, e);
                render_error(res, e);
                return;
            }
        }
    };

    // Get user role in workspace
    let user_role = match app_state
        .workspace_repo
        .get_membership(workspace.id, &user_id)
        .await
    {
        Ok(Some(membership)) => Some(membership.role),
        _ => None,
    };

    // Create default user settings (not persisted yet since we don't have a user settings table)
    let user_settings = UserSettingsDto {
        email: user_email,
        ..Default::default()
    };

    render_success(
        res,
        InitializationResponse {
            workspace: Some(InitWorkspaceDto::from_entity(&workspace, user_role)),
            user_settings: Some(user_settings),
            message: "User initialization completed successfully".to_string(),
        },
    );
}

/// Check if user needs to go through initialization.
#[endpoint(
    operation_id = "get_initialization_status",
    summary = "Get initialization status",
    description = "Check if user needs to go through initialization",
    tags("User"),
    responses(
        (status_code = 200, description = "Initialization status indicating if user needs setup", body = InitializationStatusResponse)
    )
)]
pub async fn get_initialization_status(depot: &mut Depot, res: &mut Response) {
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

    tracing::info!("Checking initialization status for user {}", user_id);

    // Check if user has any workspaces
    let has_workspaces = match app_state.workspace_repo.list_by_user(&user_id).await {
        Ok(ws) => !ws.is_empty(),
        Err(e) => {
            tracing::error!("Error checking workspaces for user {}: {:?}", user_id, e);
            false
        }
    };

    // Check if user has any domains
    let has_domains = match app_state.domain_repo.list_by_owner(&user_id).await {
        Ok(domains) => !domains.is_empty(),
        Err(e) => {
            tracing::error!("Error checking domains for user {}: {:?}", user_id, e);
            false
        }
    };

    // User settings not implemented yet, assume true for now
    let has_user_settings = true;

    // User needs initialization if they have no workspaces OR no domains OR no user settings
    let needs_initialization = !has_workspaces || !has_domains || !has_user_settings;

    tracing::info!(
        "User {} initialization status: needs_initialization={}, has_workspaces={}, has_domains={}, has_user_settings={}",
        user_id,
        needs_initialization,
        has_workspaces,
        has_domains,
        has_user_settings
    );

    render_success(
        res,
        InitializationStatusResponse {
            needs_initialization,
            has_workspaces,
            has_domains,
            has_user_settings,
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
