//! Workspace repository trait.

use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::entities::{Result, UserWorkspace, Workspace};

/// Workspace repository trait for database operations.
#[async_trait]
pub trait WorkspaceRepository: Send + Sync {
    /// Get workspace by ID.
    async fn get_by_id(&self, id: Uuid) -> Result<Option<Workspace>>;

    /// List workspaces for a user.
    async fn list_by_user(&self, user_id: &str) -> Result<Vec<Workspace>>;

    /// Create a new workspace.
    async fn create(&self, workspace: &Workspace) -> Result<Workspace>;

    /// Update an existing workspace.
    async fn update(&self, workspace: &Workspace) -> Result<Workspace>;

    /// Delete a workspace by ID.
    async fn delete(&self, id: Uuid) -> Result<()>;

    /// Get user's system workspace (auto-created).
    async fn get_system_workspace(&self, user_id: &str) -> Result<Option<Workspace>>;

    /// Create system workspace for user.
    async fn create_system_workspace(&self, user_id: &str) -> Result<Workspace>;

    /// Add user to workspace.
    async fn add_member(&self, membership: &UserWorkspace) -> Result<UserWorkspace>;

    /// Remove user from workspace.
    async fn remove_member(&self, workspace_id: Uuid, user_id: &str) -> Result<()>;

    /// Get user's membership in workspace.
    async fn get_membership(
        &self,
        workspace_id: Uuid,
        user_id: &str,
    ) -> Result<Option<UserWorkspace>>;

    /// List workspace members.
    async fn list_members(&self, workspace_id: Uuid) -> Result<Vec<UserWorkspace>>;

    /// Update member role.
    async fn update_member_role(
        &self,
        workspace_id: Uuid,
        user_id: &str,
        role: &str,
    ) -> Result<()>;

    /// Check if user has access to workspace.
    async fn user_has_access(&self, workspace_id: Uuid, user_id: &str) -> Result<bool>;
}
