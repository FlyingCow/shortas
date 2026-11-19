//! Workspace invitation repository trait.

use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::entities::{Result, WorkspaceInvitation};

/// Repository for workspace invitations.
#[async_trait]
pub trait InvitationRepository: Send + Sync {
    /// Create a new invitation.
    async fn create(&self, invitation: &WorkspaceInvitation) -> Result<WorkspaceInvitation>;

    /// Get an invitation by ID.
    async fn get_by_id(&self, id: Uuid) -> Result<Option<WorkspaceInvitation>>;

    /// Get an invitation by its token hash.
    async fn get_by_token_hash(&self, token_hash: &str) -> Result<Option<WorkspaceInvitation>>;

    /// Get the current pending invitation for a workspace/email pair, if any.
    async fn get_pending_for_email(
        &self,
        workspace_id: Uuid,
        email: &str,
    ) -> Result<Option<WorkspaceInvitation>>;

    /// List pending invitations for a workspace.
    async fn list_pending_by_workspace(&self, workspace_id: Uuid) -> Result<Vec<WorkspaceInvitation>>;

    /// Mark an invitation as accepted.
    async fn mark_accepted(&self, id: Uuid) -> Result<()>;

    /// Mark an invitation as declined.
    async fn mark_declined(&self, id: Uuid) -> Result<()>;

    /// Mark an invitation as revoked.
    async fn mark_revoked(&self, id: Uuid) -> Result<()>;

    /// Expire any pending invitations past their `expires_at`. Returns count expired.
    async fn expire_overdue(&self) -> Result<i64>;
}
