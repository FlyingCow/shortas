//! Workspaces controller for multi-tenancy management.

use salvo::prelude::*;
use uuid::Uuid;

use crate::application::dto::{
    AddMemberDto, CreateWorkspaceDto, UpdateMemberRoleDto, UpdateWorkspaceDto,
    WorkspaceDto, WorkspaceMemberDto,
};
use crate::domain::entities::ApiError;
use crate::presentation::middleware::{
    render_created, render_error, render_no_content, render_success, DepotExt, UserExt,
};

use super::cors_preflight;

/// Build workspaces controller router.
pub fn workspaces_controller() -> Router {
    Router::with_path("workspaces")
        .get(list_workspaces)
        .post(create_workspace)
        .options(cors_preflight)
        .push(
            Router::with_path("{id}")
                .get(get_workspace)
                .put(update_workspace)
                .delete(delete_workspace)
                .options(cors_preflight)
                .push(
                    Router::with_path("members")
                        .get(list_members)
                        .post(add_member)
                        .options(cors_preflight)
                        .push(
                            Router::with_path("{user_id}")
                                .put(update_member_role)
                                .delete(remove_member)
                                .options(cors_preflight),
                        ),
                ),
        )
}

/// List workspaces for the current user.
#[endpoint(
    operation_id = "list_workspaces",
    summary = "List workspaces",
    description = "List all workspaces the current user is a member of",
    tags("Workspaces"),
    responses(
        (status_code = 200, description = "Workspaces list", body = Vec<WorkspaceDto>)
    )
)]
pub async fn list_workspaces(depot: &mut Depot, res: &mut Response) {
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

    match app_state.workspace_repo.list_by_user(&user_id).await {
        Ok(workspaces) => {
            let mut dtos = Vec::new();
            for ws in workspaces {
                let member_count = app_state
                    .workspace_repo
                    .list_members(ws.id)
                    .await
                    .map(|m| m.len() as i32)
                    .unwrap_or(0);
                let route_count = app_state
                    .route_repo
                    .count_by_workspace(ws.id)
                    .await
                    .unwrap_or(0);
                dtos.push(WorkspaceDto::from_entity(ws).with_counts(member_count, route_count));
            }
            render_success(res, dtos);
        }
        Err(e) => render_error(res, e),
    }
}

/// Get a workspace by ID.
#[endpoint(
    operation_id = "get_workspace",
    summary = "Get workspace",
    description = "Get a workspace by its ID",
    tags("Workspaces"),
    parameters(
        ("id" = String, Path, description = "Workspace ID")
    ),
    responses(
        (status_code = 200, description = "Workspace details", body = WorkspaceDto),
        (status_code = 404, description = "Workspace not found")
    )
)]
pub async fn get_workspace(req: &mut Request, depot: &mut Depot, res: &mut Response) {
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

    let id_str: String = req.param("id").unwrap_or_default();
    let id = match Uuid::parse_str(&id_str) {
        Ok(id) => id,
        Err(_) => {
            render_error(res, ApiError::validation("Invalid workspace ID"));
            return;
        }
    };

    if !app_state.workspace_repo.user_has_access(id, &user_id).await.unwrap_or(false) {
        render_error(res, ApiError::forbidden());
        return;
    }

    match app_state.workspace_repo.get_by_id(id).await {
        Ok(Some(ws)) => {
            let member_count = app_state
                .workspace_repo
                .list_members(ws.id)
                .await
                .map(|m| m.len() as i32)
                .unwrap_or(0);
            let route_count = app_state
                .route_repo
                .count_by_workspace(ws.id)
                .await
                .unwrap_or(0);
            render_success(res, WorkspaceDto::from_entity(ws).with_counts(member_count, route_count));
        }
        Ok(None) => render_error(res, ApiError::not_found("Workspace", &id_str)),
        Err(e) => render_error(res, e),
    }
}

/// Create a new workspace.
#[endpoint(
    operation_id = "create_workspace",
    summary = "Create workspace",
    description = "Create a new workspace",
    tags("Workspaces"),
    request_body(content = CreateWorkspaceDto, description = "Workspace to create"),
    responses(
        (status_code = 201, description = "Workspace created", body = WorkspaceDto)
    )
)]
pub async fn create_workspace(req: &mut Request, depot: &mut Depot, res: &mut Response) {
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

    let dto: CreateWorkspaceDto = match req.parse_json().await {
        Ok(dto) => dto,
        Err(e) => {
            render_error(res, ApiError::validation(e.to_string()));
            return;
        }
    };

    let workspace = dto.to_entity();

    match app_state.workspace_repo.create(&workspace).await {
        Ok(created) => {
            // Add creator as owner
            let membership = crate::domain::entities::UserWorkspace::new(
                user_id,
                created.id,
                "Owner".to_string(),
            );
            let _ = app_state.workspace_repo.add_member(&membership).await;

            render_created(res, WorkspaceDto::from_entity(created).with_counts(1, 0));
        }
        Err(e) => render_error(res, e),
    }
}

/// Update a workspace.
#[endpoint(
    operation_id = "update_workspace",
    summary = "Update workspace",
    description = "Update an existing workspace",
    tags("Workspaces"),
    parameters(
        ("id" = String, Path, description = "Workspace ID")
    ),
    request_body(content = UpdateWorkspaceDto, description = "Workspace updates"),
    responses(
        (status_code = 200, description = "Workspace updated", body = WorkspaceDto)
    )
)]
pub async fn update_workspace(req: &mut Request, depot: &mut Depot, res: &mut Response) {
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

    let id_str: String = req.param("id").unwrap_or_default();
    let id = match Uuid::parse_str(&id_str) {
        Ok(id) => id,
        Err(_) => {
            render_error(res, ApiError::validation("Invalid workspace ID"));
            return;
        }
    };

    // Check if user is admin
    let membership = match app_state.workspace_repo.get_membership(id, &user_id).await {
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

    let dto: UpdateWorkspaceDto = match req.parse_json().await {
        Ok(dto) => dto,
        Err(e) => {
            render_error(res, ApiError::validation(e.to_string()));
            return;
        }
    };

    let workspace = match app_state.workspace_repo.get_by_id(id).await {
        Ok(Some(ws)) => ws,
        Ok(None) => {
            render_error(res, ApiError::not_found("Workspace", &id_str));
            return;
        }
        Err(e) => {
            render_error(res, e);
            return;
        }
    };

    // Don't allow updating system workspaces
    if workspace.is_system() {
        render_error(res, ApiError::validation("Cannot update system workspace"));
        return;
    }

    let updated = dto.apply_to(workspace);

    match app_state.workspace_repo.update(&updated).await {
        Ok(saved) => render_success(res, WorkspaceDto::from_entity(saved)),
        Err(e) => render_error(res, e),
    }
}

/// Delete a workspace.
#[endpoint(
    operation_id = "delete_workspace",
    summary = "Delete workspace",
    description = "Delete a workspace",
    tags("Workspaces"),
    parameters(
        ("id" = String, Path, description = "Workspace ID")
    ),
    responses(
        (status_code = 204, description = "Workspace deleted")
    )
)]
pub async fn delete_workspace(req: &mut Request, depot: &mut Depot, res: &mut Response) {
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

    let id_str: String = req.param("id").unwrap_or_default();
    let id = match Uuid::parse_str(&id_str) {
        Ok(id) => id,
        Err(_) => {
            render_error(res, ApiError::validation("Invalid workspace ID"));
            return;
        }
    };

    // Check if user is owner
    let membership = match app_state.workspace_repo.get_membership(id, &user_id).await {
        Ok(Some(m)) => m,
        _ => {
            render_error(res, ApiError::forbidden());
            return;
        }
    };

    if !membership.is_owner() {
        render_error(res, ApiError::forbidden());
        return;
    }

    let workspace = match app_state.workspace_repo.get_by_id(id).await {
        Ok(Some(ws)) => ws,
        Ok(None) => {
            render_error(res, ApiError::not_found("Workspace", &id_str));
            return;
        }
        Err(e) => {
            render_error(res, e);
            return;
        }
    };

    // Don't allow deleting system workspaces
    if workspace.is_system() {
        render_error(res, ApiError::validation("Cannot delete system workspace"));
        return;
    }

    match app_state.workspace_repo.delete(id).await {
        Ok(()) => render_no_content(res),
        Err(e) => render_error(res, e),
    }
}

/// List workspace members.
#[endpoint(
    operation_id = "list_members",
    summary = "List members",
    description = "List all members of a workspace",
    tags("Workspaces"),
    parameters(
        ("id" = String, Path, description = "Workspace ID")
    ),
    responses(
        (status_code = 200, description = "Members list", body = Vec<WorkspaceMemberDto>)
    )
)]
pub async fn list_members(req: &mut Request, depot: &mut Depot, res: &mut Response) {
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

    let id_str: String = req.param("id").unwrap_or_default();
    let id = match Uuid::parse_str(&id_str) {
        Ok(id) => id,
        Err(_) => {
            render_error(res, ApiError::validation("Invalid workspace ID"));
            return;
        }
    };

    if !app_state.workspace_repo.user_has_access(id, &user_id).await.unwrap_or(false) {
        render_error(res, ApiError::forbidden());
        return;
    }

    match app_state.workspace_repo.list_members(id).await {
        Ok(members) => {
            let dtos: Vec<WorkspaceMemberDto> = members
                .into_iter()
                .map(WorkspaceMemberDto::from_entity)
                .collect();
            render_success(res, dtos);
        }
        Err(e) => render_error(res, e),
    }
}

/// Add a member to a workspace.
#[endpoint(
    operation_id = "add_member",
    summary = "Add member",
    description = "Add a new member to a workspace",
    tags("Workspaces"),
    parameters(
        ("id" = String, Path, description = "Workspace ID")
    ),
    request_body(content = AddMemberDto, description = "Member to add"),
    responses(
        (status_code = 201, description = "Member added", body = WorkspaceMemberDto)
    )
)]
pub async fn add_member(req: &mut Request, depot: &mut Depot, res: &mut Response) {
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

    let id_str: String = req.param("id").unwrap_or_default();
    let id = match Uuid::parse_str(&id_str) {
        Ok(id) => id,
        Err(_) => {
            render_error(res, ApiError::validation("Invalid workspace ID"));
            return;
        }
    };

    // Check if user is admin
    let membership = match app_state.workspace_repo.get_membership(id, &user_id).await {
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

    let dto: AddMemberDto = match req.parse_json().await {
        Ok(dto) => dto,
        Err(e) => {
            render_error(res, ApiError::validation(e.to_string()));
            return;
        }
    };

    let new_member = dto.to_entity(id);

    match app_state.workspace_repo.add_member(&new_member).await {
        Ok(added) => render_created(res, WorkspaceMemberDto::from_entity(added)),
        Err(e) => render_error(res, e),
    }
}

/// Update a member's role.
#[endpoint(
    operation_id = "update_member_role",
    summary = "Update member role",
    description = "Update a member's role in a workspace",
    tags("Workspaces"),
    parameters(
        ("id" = String, Path, description = "Workspace ID"),
        ("user_id" = String, Path, description = "User ID")
    ),
    request_body(content = UpdateMemberRoleDto, description = "New role"),
    responses(
        (status_code = 200, description = "Role updated")
    )
)]
pub async fn update_member_role(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let app_state = match depot.app_state() {
        Ok(state) => state,
        Err(e) => {
            render_error(res, ApiError::internal(e.to_string()));
            return;
        }
    };

    let current_user_id = match depot.user_id() {
        Ok(id) => id,
        Err(e) => {
            render_error(res, ApiError::unauthorized(e.to_string()));
            return;
        }
    };

    let id_str: String = req.param("id").unwrap_or_default();
    let id = match Uuid::parse_str(&id_str) {
        Ok(id) => id,
        Err(_) => {
            render_error(res, ApiError::validation("Invalid workspace ID"));
            return;
        }
    };

    let target_user_id: String = req.param("user_id").unwrap_or_default();

    // Check if user is owner
    let membership = match app_state.workspace_repo.get_membership(id, &current_user_id).await {
        Ok(Some(m)) => m,
        _ => {
            render_error(res, ApiError::forbidden());
            return;
        }
    };

    if !membership.is_owner() {
        render_error(res, ApiError::forbidden());
        return;
    }

    let dto: UpdateMemberRoleDto = match req.parse_json().await {
        Ok(dto) => dto,
        Err(e) => {
            render_error(res, ApiError::validation(e.to_string()));
            return;
        }
    };

    match app_state
        .workspace_repo
        .update_member_role(id, &target_user_id, &dto.role)
        .await
    {
        Ok(()) => {
            res.render(Json(serde_json::json!({ "message": "Role updated" })));
        }
        Err(e) => render_error(res, e),
    }
}

/// Remove a member from a workspace.
#[endpoint(
    operation_id = "remove_member",
    summary = "Remove member",
    description = "Remove a member from a workspace",
    tags("Workspaces"),
    parameters(
        ("id" = String, Path, description = "Workspace ID"),
        ("user_id" = String, Path, description = "User ID")
    ),
    responses(
        (status_code = 204, description = "Member removed")
    )
)]
pub async fn remove_member(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let app_state = match depot.app_state() {
        Ok(state) => state,
        Err(e) => {
            render_error(res, ApiError::internal(e.to_string()));
            return;
        }
    };

    let current_user_id = match depot.user_id() {
        Ok(id) => id,
        Err(e) => {
            render_error(res, ApiError::unauthorized(e.to_string()));
            return;
        }
    };

    let id_str: String = req.param("id").unwrap_or_default();
    let id = match Uuid::parse_str(&id_str) {
        Ok(id) => id,
        Err(_) => {
            render_error(res, ApiError::validation("Invalid workspace ID"));
            return;
        }
    };

    let target_user_id: String = req.param("user_id").unwrap_or_default();

    // Check if user is admin
    let membership = match app_state.workspace_repo.get_membership(id, &current_user_id).await {
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

    // Can't remove owner
    if let Ok(Some(target)) = app_state.workspace_repo.get_membership(id, &target_user_id).await {
        if target.is_owner() {
            render_error(res, ApiError::validation("Cannot remove workspace owner"));
            return;
        }
    }

    match app_state.workspace_repo.remove_member(id, &target_user_id).await {
        Ok(()) => render_no_content(res),
        Err(e) => render_error(res, e),
    }
}
