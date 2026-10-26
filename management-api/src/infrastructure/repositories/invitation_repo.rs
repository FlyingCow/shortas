//! Workspace invitation repository implementation using SQLx.

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{PgPool, Row};
use uuid::Uuid;

use crate::domain::entities::{ApiError, InvitationStatus, Result, WorkspaceInvitation};
use crate::domain::traits::InvitationRepository;

/// PostgreSQL workspace invitation repository.
pub struct PgInvitationRepository {
    pool: PgPool,
}

impl PgInvitationRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    fn map_row(row: &sqlx::postgres::PgRow) -> Result<WorkspaceInvitation> {
        let id: Uuid = row.try_get("id").map_err(|e| ApiError::internal(e.to_string()))?;
        let workspace_id: Uuid = row.try_get("workspace_id").map_err(|e| ApiError::internal(e.to_string()))?;
        let invited_email: String = row.try_get("invited_email").unwrap_or_default();
        let invited_user_id: Option<String> = row.try_get("invited_user_id").ok();
        let role: String = row.try_get("role").unwrap_or_else(|_| "Member".to_string());
        let invited_by: String = row.try_get("invited_by").unwrap_or_default();
        let token_hash: String = row.try_get("token_hash").unwrap_or_default();
        let status_str: String = row.try_get("status").unwrap_or_else(|_| "Pending".to_string());
        let expires_at: DateTime<Utc> = row.try_get("expires_at").unwrap_or_else(|_| Utc::now());
        let created_at: DateTime<Utc> = row.try_get("created_at").unwrap_or_else(|_| Utc::now());
        let responded_at: Option<DateTime<Utc>> = row.try_get("responded_at").ok();

        Ok(WorkspaceInvitation {
            id,
            workspace_id,
            invited_email,
            invited_user_id,
            role,
            invited_by,
            token_hash,
            status: InvitationStatus::from_str(&status_str),
            expires_at,
            created_at,
            responded_at,
        })
    }

    async fn set_status(&self, id: Uuid, status: InvitationStatus) -> Result<()> {
        sqlx::query(
            "UPDATE workspace_invitations SET status = $2, responded_at = NOW() WHERE id = $1",
        )
        .bind(id)
        .bind(status.as_str())
        .execute(&self.pool)
        .await
        .map_err(|e| ApiError::internal(e.to_string()))?;

        Ok(())
    }
}

#[async_trait]
impl InvitationRepository for PgInvitationRepository {
    async fn create(&self, invitation: &WorkspaceInvitation) -> Result<WorkspaceInvitation> {
        sqlx::query(
            r#"
            INSERT INTO workspace_invitations
                (id, workspace_id, invited_email, invited_user_id, role, invited_by, token_hash, status, expires_at, created_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            "#,
        )
        .bind(invitation.id)
        .bind(invitation.workspace_id)
        .bind(&invitation.invited_email)
        .bind(&invitation.invited_user_id)
        .bind(&invitation.role)
        .bind(&invitation.invited_by)
        .bind(&invitation.token_hash)
        .bind(invitation.status.as_str())
        .bind(invitation.expires_at)
        .bind(invitation.created_at)
        .execute(&self.pool)
        .await
        .map_err(|e| ApiError::internal(e.to_string()))?;

        self.get_by_id(invitation.id)
            .await?
            .ok_or_else(|| ApiError::internal("Failed to retrieve created invitation"))
    }

    async fn get_by_id(&self, id: Uuid) -> Result<Option<WorkspaceInvitation>> {
        let row = sqlx::query("SELECT * FROM workspace_invitations WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| ApiError::internal(e.to_string()))?;

        match row {
            Some(r) => Ok(Some(Self::map_row(&r)?)),
            None => Ok(None),
        }
    }

    async fn get_by_token_hash(&self, token_hash: &str) -> Result<Option<WorkspaceInvitation>> {
        let row = sqlx::query("SELECT * FROM workspace_invitations WHERE token_hash = $1")
            .bind(token_hash)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| ApiError::internal(e.to_string()))?;

        match row {
            Some(r) => Ok(Some(Self::map_row(&r)?)),
            None => Ok(None),
        }
    }

    async fn get_pending_for_email(
        &self,
        workspace_id: Uuid,
        email: &str,
    ) -> Result<Option<WorkspaceInvitation>> {
        let row = sqlx::query(
            r#"
            SELECT * FROM workspace_invitations
            WHERE workspace_id = $1 AND LOWER(invited_email) = LOWER($2) AND status = 'Pending'
            "#,
        )
        .bind(workspace_id)
        .bind(email)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| ApiError::internal(e.to_string()))?;

        match row {
            Some(r) => Ok(Some(Self::map_row(&r)?)),
            None => Ok(None),
        }
    }

    async fn list_pending_by_workspace(&self, workspace_id: Uuid) -> Result<Vec<WorkspaceInvitation>> {
        let rows = sqlx::query(
            "SELECT * FROM workspace_invitations WHERE workspace_id = $1 AND status = 'Pending' ORDER BY created_at DESC",
        )
        .bind(workspace_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| ApiError::internal(e.to_string()))?;

        Ok(rows.iter().filter_map(|r| Self::map_row(r).ok()).collect())
    }

    async fn mark_accepted(&self, id: Uuid) -> Result<()> {
        self.set_status(id, InvitationStatus::Accepted).await
    }

    async fn mark_declined(&self, id: Uuid) -> Result<()> {
        self.set_status(id, InvitationStatus::Declined).await
    }

    async fn mark_revoked(&self, id: Uuid) -> Result<()> {
        self.set_status(id, InvitationStatus::Revoked).await
    }

    async fn expire_overdue(&self) -> Result<i64> {
        let result = sqlx::query(
            "UPDATE workspace_invitations SET status = 'Expired' WHERE status = 'Pending' AND expires_at <= NOW()",
        )
        .execute(&self.pool)
        .await
        .map_err(|e| ApiError::internal(e.to_string()))?;

        Ok(result.rows_affected() as i64)
    }
}
