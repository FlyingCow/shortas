//! Workspace invitation controller: invite-by-email, accept/decline, revoke.

use salvo::prelude::*;
use uuid::Uuid;

use crate::application::dto::{
    CreateInvitationDto, InvitationDto, InvitationPreviewDto, WorkspaceMemberDto,
};
use crate::domain::entities::{
    ApiError, NotificationRecipient, OutboxMessage, UserWorkspace, WorkspaceInvitation,
};
use crate::presentation::middleware::{
    render_created, render_error, render_no_content, render_success, DepotExt, UserExt,
};

use super::cors_preflight;

/// Routes nested under `/workspaces/{id}/invitations` (admin-only management).
pub fn workspace_invitations_controller() -> Router {
    Router::with_path("invitations")
        .get(list_invitations)
        .post(create_invitation)
        .options(cors_preflight)
        .push(
            Router::with_path("{invitation_id}")
                .delete(revoke_invitation)
                .options(cors_preflight),
        )
}

/// Standalone `/invitations/...` routes for accepting/declining.
/// `by_token_preview` must be mounted on the PUBLIC (unauthenticated) router.
pub fn invitations_controller() -> Router {
    Router::with_path("invitations")
        .options(cors_preflight)
        .push(
            Router::with_path("{id}")
                .push(Router::with_path("accept").post(accept_invitation).options(cors_preflight))
                .push(Router::with_path("decline").post(decline_invitation).options(cors_preflight)),
        )
        .push(
            Router::with_path("by-token/{token}/accept")
                .post(accept_invitation_by_token)
                .options(cors_preflight),
        )
}

/// Public preview route: `GET /invitations/by-token/{token}`. No auth required.
pub fn invitations_public_controller() -> Router {
    Router::with_path("invitations/by-token/{token}")
        .get(preview_invitation_by_token)
        .options(cors_preflight)
}

fn invalid_role_error(role: &str) -> ApiError {
    ApiError::validation(format!(
        "Invalid role '{}': must be 'Admin' or 'Member'",
        role
    ))
}

/// Invite a user to a workspace by email.
#[endpoint(
    operation_id = "create_invitation",
    summary = "Invite member by email",
    description = "Invite a user to join a workspace by email. Sends an email and, if the user already has an account, an in-app notification.",
    tags("Invitations"),
    parameters(("id" = String, Path, description = "Workspace ID")),
    request_body(content = CreateInvitationDto, description = "Invitee email and role"),
    responses((status_code = 201, description = "Invitation created", body = InvitationDto))
)]
pub async fn create_invitation(req: &mut Request, depot: &mut Depot, res: &mut Response) {
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

    let workspace_id_str: String = req.param("id").unwrap_or_default();
    let workspace_id = match Uuid::parse_str(&workspace_id_str) {
        Ok(id) => id,
        Err(_) => {
            render_error(res, ApiError::validation("Invalid workspace ID"));
            return;
        }
    };

    let membership = match app_state.workspace_repo.get_membership(workspace_id, &user_id).await {
        Ok(Some(m)) => m,
        _ => {
            render_error(res, ApiError::forbidden());
            return;
        }
    };
    if !membership.is_admin() {
        render_error(res, ApiError::forbidden());
        return;
    }

    let dto: CreateInvitationDto = match req.parse_json().await {
        Ok(dto) => dto,
        Err(e) => {
            render_error(res, ApiError::validation(e.to_string()));
            return;
        }
    };

    let email = dto.email.trim().to_lowercase();
    if email.is_empty() || !email.contains('@') {
        render_error(res, ApiError::validation("A valid email is required"));
        return;
    }
    if email.len() > 255 {
        render_error(res, ApiError::validation("Email must be 255 characters or fewer"));
        return;
    }
    if dto.role != "Admin" && dto.role != "Member" {
        render_error(res, invalid_role_error(&dto.role));
        return;
    }

    let workspace = match app_state.workspace_repo.get_by_id(workspace_id).await {
        Ok(Some(ws)) => ws,
        _ => {
            render_error(res, ApiError::not_found("Workspace", &workspace_id_str));
            return;
        }
    };

    // Already resolvable to an existing user? Make sure they're not already a member.
    let existing_user = app_state.user_repo.get_by_email(&email).await.ok().flatten();
    if let Some(ref u) = existing_user {
        if app_state
            .workspace_repo
            .user_has_access(workspace_id, &u.id)
            .await
            .unwrap_or(false)
        {
            render_error(res, ApiError::conflict("This user is already a member of the workspace"));
            return;
        }
    }

    if app_state
        .invitation_repo
        .get_pending_for_email(workspace_id, &email)
        .await
        .ok()
        .flatten()
        .is_some()
    {
        render_error(res, ApiError::conflict("There is already a pending invitation for this email"));
        return;
    }

    let (raw_token, token_hash) = WorkspaceInvitation::generate_token();
    let invited_user_id = existing_user.map(|u| u.id);
    let invitation = WorkspaceInvitation::new(
        workspace_id,
        email.clone(),
        invited_user_id.clone(),
        dto.role.clone(),
        user_id.clone(),
        token_hash,
    );

    let created = match app_state.invitation_repo.create(&invitation).await {
        Ok(created) => created,
        Err(e) => {
            render_error(res, e);
            return;
        }
    };

    let inviter_name = depot
        .user_profile()
        .ok()
        .and_then(|p| p.name.clone().or_else(|| p.email.clone()))
        .unwrap_or_else(|| "Someone".to_string());

    let invite_url = format!(
        "{}/invite/{}",
        app_state.settings.email.dashboard_base_url.trim_end_matches('/'),
        raw_token
    );
    // The in-app notification shows "User" for the Member role (matches the
    // dashboard's own terminology); email keeps the raw role name.
    let role_label = if dto.role == "Member" { "User" } else { dto.role.as_str() };

    let notify_msg = OutboxMessage::notify(
        "workspace.invite_created",
        NotificationRecipient {
            user_id: invited_user_id,
            email: Some(email.clone()),
        },
        serde_json::json!({
            "invitation_id": created.id,
            "workspace_id": workspace_id,
            "workspace_name": workspace.name,
            "role": dto.role,
            "role_label": role_label,
            "inviter_name": inviter_name,
            "invite_url": invite_url,
        }),
    );
    let _ = app_state.outbox_repo.create(&notify_msg).await;

    render_created(res, InvitationDto::from_entity(created));
}

/// List pending invitations for a workspace.
#[endpoint(
    operation_id = "list_invitations",
    summary = "List pending invitations",
    tags("Invitations"),
    parameters(("id" = String, Path, description = "Workspace ID")),
    responses((status_code = 200, description = "Pending invitations", body = Vec<InvitationDto>))
)]
pub async fn list_invitations(req: &mut Request, depot: &mut Depot, res: &mut Response) {
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

    let workspace_id_str: String = req.param("id").unwrap_or_default();
    let workspace_id = match Uuid::parse_str(&workspace_id_str) {
        Ok(id) => id,
        Err(_) => {
            render_error(res, ApiError::validation("Invalid workspace ID"));
            return;
        }
    };

    let membership = match app_state.workspace_repo.get_membership(workspace_id, &user_id).await {
        Ok(Some(m)) => m,
        _ => {
            render_error(res, ApiError::forbidden());
            return;
        }
    };
    if !membership.is_admin() {
        render_error(res, ApiError::forbidden());
        return;
    }

    match app_state.invitation_repo.list_pending_by_workspace(workspace_id).await {
        Ok(invitations) => {
            render_success(res, invitations.into_iter().map(InvitationDto::from_entity).collect::<Vec<_>>());
        }
        Err(e) => render_error(res, e),
    }
}

/// Revoke a pending invitation.
#[endpoint(
    operation_id = "revoke_invitation",
    summary = "Revoke invitation",
    tags("Invitations"),
    parameters(
        ("id" = String, Path, description = "Workspace ID"),
        ("invitation_id" = String, Path, description = "Invitation ID")
    ),
    responses((status_code = 204, description = "Invitation revoked"))
)]
pub async fn revoke_invitation(req: &mut Request, depot: &mut Depot, res: &mut Response) {
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

    let workspace_id_str: String = req.param("id").unwrap_or_default();
    let workspace_id = match Uuid::parse_str(&workspace_id_str) {
        Ok(id) => id,
        Err(_) => {
            render_error(res, ApiError::validation("Invalid workspace ID"));
            return;
        }
    };

    let membership = match app_state.workspace_repo.get_membership(workspace_id, &user_id).await {
        Ok(Some(m)) => m,
        _ => {
            render_error(res, ApiError::forbidden());
            return;
        }
    };
    if !membership.is_admin() {
        render_error(res, ApiError::forbidden());
        return;
    }

    let invitation_id_str: String = req.param("invitation_id").unwrap_or_default();
    let invitation_id = match Uuid::parse_str(&invitation_id_str) {
        Ok(id) => id,
        Err(_) => {
            render_error(res, ApiError::validation("Invalid invitation ID"));
            return;
        }
    };

    match app_state.invitation_repo.get_by_id(invitation_id).await {
        Ok(Some(inv)) if inv.workspace_id == workspace_id => {
            match app_state.invitation_repo.mark_revoked(invitation_id).await {
                Ok(()) => render_no_content(res),
                Err(e) => render_error(res, e),
            }
        }
        Ok(_) => render_error(res, ApiError::not_found("Invitation", &invitation_id_str)),
        Err(e) => render_error(res, e),
    }
}

/// Public, unauthenticated preview of an invitation by its raw token.
#[endpoint(
    operation_id = "preview_invitation_by_token",
    summary = "Preview invitation by token",
    tags("Invitations"),
    parameters(("token" = String, Path, description = "Raw invitation token")),
    responses(
        (status_code = 200, description = "Invitation preview", body = InvitationPreviewDto),
        (status_code = 404, description = "Invitation not found"),
        (status_code = 409, description = "Invitation no longer valid")
    )
)]
pub async fn preview_invitation_by_token(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let app_state = match depot.app_state() {
        Ok(state) => state,
        Err(e) => {
            render_error(res, ApiError::internal(e.to_string()));
            return;
        }
    };

    let token: String = req.param("token").unwrap_or_default();
    let token_hash = WorkspaceInvitation::hash_token(&token);

    let invitation = match app_state.invitation_repo.get_by_token_hash(&token_hash).await {
        Ok(Some(inv)) => inv,
        Ok(None) => {
            render_error(res, ApiError::not_found("Invitation", "token"));
            return;
        }
        Err(e) => {
            render_error(res, e);
            return;
        }
    };

    let workspace_name = app_state
        .workspace_repo
        .get_by_id(invitation.workspace_id)
        .await
        .ok()
        .flatten()
        .map(|w| w.name)
        .unwrap_or_else(|| "a workspace".to_string());

    let inviter_name = app_state
        .user_repo
        .get_by_id(&invitation.invited_by)
        .await
        .ok()
        .flatten()
        .and_then(|u| u.name.or(Some(u.email)))
        .unwrap_or_else(|| "Someone".to_string());

    render_success(
        res,
        InvitationPreviewDto {
            workspace_name,
            inviter_name,
            role: invitation.role,
            invited_email: invitation.invited_email,
            status: invitation.status.as_str().to_string(),
        },
    );
}

/// Shared accept logic: validates the invitation is pending and belongs to the
/// authenticated user's email, creates the membership, and marks it accepted.
async fn accept_invitation_internal(
    depot: &mut Depot,
    res: &mut Response,
    invitation: WorkspaceInvitation,
) {
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
    let user_email = depot.user_email().unwrap_or_default();

    if !invitation.is_pending() {
        render_error(res, ApiError::conflict("This invitation is no longer valid"));
        return;
    }

    let email_matches = invitation.invited_email.to_lowercase() == user_email.to_lowercase();
    let id_matches = invitation.invited_user_id.as_deref() == Some(user_id.as_str());
    if !email_matches && !id_matches {
        render_error(res, ApiError::forbidden());
        return;
    }

    if app_state
        .workspace_repo
        .user_has_access(invitation.workspace_id, &user_id)
        .await
        .unwrap_or(false)
    {
        // Already a member (e.g. added another way, or this invite was
        // accepted once already from another tab) - don't error, just
        // finish marking the invitation itself as accepted.
        if let Err(e) = app_state.invitation_repo.mark_accepted(invitation.id).await {
            tracing::warn!("Failed to mark already-member invitation {} accepted: {}", invitation.id, e);
        }
        let _ = app_state.notification_repo.mark_actioned_by_invitation(invitation.id).await;
        render_error(res, ApiError::conflict("You're already a member of this workspace"));
        return;
    }

    let membership = UserWorkspace::new(user_id, invitation.workspace_id, invitation.role.clone());
    let created = match app_state.workspace_repo.add_member(&membership).await {
        Ok(m) => m,
        Err(e) => {
            render_error(res, e);
            return;
        }
    };

    if let Err(e) = app_state.invitation_repo.mark_accepted(invitation.id).await {
        tracing::warn!("Membership created but failed to mark invitation {} accepted: {}", invitation.id, e);
    }
    let _ = app_state.notification_repo.mark_actioned_by_invitation(invitation.id).await;

    render_success(res, WorkspaceMemberDto::from_entity(created));
}

/// Accept an invitation (already authenticated; seen via the notification center).
#[endpoint(
    operation_id = "accept_invitation",
    summary = "Accept invitation",
    tags("Invitations"),
    parameters(("id" = String, Path, description = "Invitation ID")),
    responses((status_code = 200, description = "Membership created", body = WorkspaceMemberDto))
)]
pub async fn accept_invitation(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let id_str: String = req.param("id").unwrap_or_default();
    let id = match Uuid::parse_str(&id_str) {
        Ok(id) => id,
        Err(_) => {
            render_error(res, ApiError::validation("Invalid invitation ID"));
            return;
        }
    };

    let app_state = match depot.app_state() {
        Ok(state) => state,
        Err(e) => {
            render_error(res, ApiError::internal(e.to_string()));
            return;
        }
    };

    match app_state.invitation_repo.get_by_id(id).await {
        Ok(Some(invitation)) => accept_invitation_internal(depot, res, invitation).await,
        Ok(None) => render_error(res, ApiError::not_found("Invitation", &id_str)),
        Err(e) => render_error(res, e),
    }
}

/// Accept an invitation via its raw email-link token.
#[endpoint(
    operation_id = "accept_invitation_by_token",
    summary = "Accept invitation by token",
    tags("Invitations"),
    parameters(("token" = String, Path, description = "Raw invitation token")),
    responses((status_code = 200, description = "Membership created", body = WorkspaceMemberDto))
)]
pub async fn accept_invitation_by_token(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let token: String = req.param("token").unwrap_or_default();
    let token_hash = WorkspaceInvitation::hash_token(&token);

    let app_state = match depot.app_state() {
        Ok(state) => state,
        Err(e) => {
            render_error(res, ApiError::internal(e.to_string()));
            return;
        }
    };

    match app_state.invitation_repo.get_by_token_hash(&token_hash).await {
        Ok(Some(invitation)) => accept_invitation_internal(depot, res, invitation).await,
        Ok(None) => render_error(res, ApiError::not_found("Invitation", "token")),
        Err(e) => render_error(res, e),
    }
}

/// Decline an invitation.
#[endpoint(
    operation_id = "decline_invitation",
    summary = "Decline invitation",
    tags("Invitations"),
    parameters(("id" = String, Path, description = "Invitation ID")),
    responses((status_code = 204, description = "Invitation declined"))
)]
pub async fn decline_invitation(req: &mut Request, depot: &mut Depot, res: &mut Response) {
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
    let user_email = depot.user_email().unwrap_or_default();

    let id_str: String = req.param("id").unwrap_or_default();
    let id = match Uuid::parse_str(&id_str) {
        Ok(id) => id,
        Err(_) => {
            render_error(res, ApiError::validation("Invalid invitation ID"));
            return;
        }
    };

    let invitation = match app_state.invitation_repo.get_by_id(id).await {
        Ok(Some(inv)) => inv,
        Ok(None) => {
            render_error(res, ApiError::not_found("Invitation", &id_str));
            return;
        }
        Err(e) => {
            render_error(res, e);
            return;
        }
    };

    let email_matches = invitation.invited_email.to_lowercase() == user_email.to_lowercase();
    let id_matches = invitation.invited_user_id.as_deref() == Some(user_id.as_str());
    if !email_matches && !id_matches {
        render_error(res, ApiError::forbidden());
        return;
    }

    match app_state.invitation_repo.mark_declined(id).await {
        Ok(()) => {
            let _ = app_state.notification_repo.mark_actioned_by_invitation(id).await;
            render_no_content(res)
        }
        Err(e) => render_error(res, e),
    }
}
