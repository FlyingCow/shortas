//! Workspace repository implementation using SQLx.

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{PgPool, Row};
use uuid::Uuid;

use crate::domain::entities::{ApiError, Result, UserWorkspace, Workspace};
use crate::domain::traits::WorkspaceRepository;

/// PostgreSQL workspace repository.
pub struct PgWorkspaceRepository {
    pool: PgPool,
}

impl PgWorkspaceRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    fn map_workspace_row(row: &sqlx::postgres::PgRow) -> Result<Workspace> {
        let id: Uuid = row.try_get("id").map_err(|e| ApiError::internal(e.to_string()))?;
        let name: String = row.try_get("name").unwrap_or_default();
        let description: String = row.try_get("description").unwrap_or_default();
        let workspace_type: String = row.try_get("type").unwrap_or_else(|_| "User".to_string());
        let created_at: Option<DateTime<Utc>> = row.try_get("created_at").ok();
        let updated_at: Option<DateTime<Utc>> = row.try_get("updated_at").ok();

        Ok(Workspace {
            id,
            name,
            description,
            workspace_type,
            created_at,
            updated_at,
        })
    }

    fn map_membership_row(row: &sqlx::postgres::PgRow) -> Result<UserWorkspace> {
        let id: Uuid = row.try_get("id").map_err(|e| ApiError::internal(e.to_string()))?;
        let user_id: String = row.try_get("user_id").unwrap_or_default();
        let workspace_id: Uuid = row.try_get("workspace_id").unwrap_or_default();
        let role: String = row.try_get("role").unwrap_or_else(|_| "Member".to_string());
        let joined_at: Option<DateTime<Utc>> = row.try_get("joined_at").ok();

        Ok(UserWorkspace {
            id,
            user_id,
            workspace_id,
            role,
            joined_at,
        })
    }
}

#[async_trait]
impl WorkspaceRepository for PgWorkspaceRepository {
    async fn get_by_id(&self, id: Uuid) -> Result<Option<Workspace>> {
        let row = sqlx::query("SELECT * FROM workspaces WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| ApiError::internal(e.to_string()))?;

        match row {
            Some(r) => Ok(Some(Self::map_workspace_row(&r)?)),
            None => Ok(None),
        }
    }

    async fn list_by_user(&self, user_id: &str) -> Result<Vec<Workspace>> {
        let rows = sqlx::query(
            r#"
            SELECT w.* FROM workspaces w
            JOIN user_workspaces uw ON w.id = uw.workspace_id
            WHERE uw.user_id = $1
            ORDER BY w.created_at
            "#,
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| ApiError::internal(e.to_string()))?;

        Ok(rows.iter().filter_map(|r| Self::map_workspace_row(r).ok()).collect())
    }

    async fn create(&self, workspace: &Workspace) -> Result<Workspace> {
        sqlx::query(
            r#"
            INSERT INTO workspaces (id, name, description, type, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6)
            "#,
        )
        .bind(workspace.id)
        .bind(&workspace.name)
        .bind(&workspace.description)
        .bind(&workspace.workspace_type)
        .bind(Utc::now())
        .bind(Utc::now())
        .execute(&self.pool)
        .await
        .map_err(|e| ApiError::internal(e.to_string()))?;

        self.get_by_id(workspace.id)
            .await?
            .ok_or_else(|| ApiError::internal("Failed to retrieve created workspace"))
    }

    async fn update(&self, workspace: &Workspace) -> Result<Workspace> {
        sqlx::query(
            r#"
            UPDATE workspaces
            SET name = $2, description = $3, updated_at = NOW()
            WHERE id = $1
            "#,
        )
        .bind(workspace.id)
        .bind(&workspace.name)
        .bind(&workspace.description)
        .execute(&self.pool)
        .await
        .map_err(|e| ApiError::internal(e.to_string()))?;

        self.get_by_id(workspace.id)
            .await?
            .ok_or_else(|| ApiError::internal("Failed to retrieve updated workspace"))
    }

    async fn delete(&self, id: Uuid) -> Result<()> {
        // Delete memberships first
        sqlx::query("DELETE FROM user_workspaces WHERE workspace_id = $1")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| ApiError::internal(e.to_string()))?;

        sqlx::query("DELETE FROM workspaces WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| ApiError::internal(e.to_string()))?;

        Ok(())
    }

    async fn get_system_workspace(&self, user_id: &str) -> Result<Option<Workspace>> {
        let row = sqlx::query(
            r#"
            SELECT w.* FROM workspaces w
            JOIN user_workspaces uw ON w.id = uw.workspace_id
            WHERE uw.user_id = $1 AND w.type = 'System'
            LIMIT 1
            "#,
        )
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| ApiError::internal(e.to_string()))?;

        match row {
            Some(r) => Ok(Some(Self::map_workspace_row(&r)?)),
            None => Ok(None),
        }
    }

    async fn create_system_workspace(&self, user_id: &str) -> Result<Workspace> {
        let workspace = Workspace::new("Personal".to_string(), "System".to_string());

        let created = self.create(&workspace).await?;

        // Add user as owner
        let membership = UserWorkspace::new(user_id.to_string(), created.id, "Owner".to_string());
        self.add_member(&membership).await?;

        Ok(created)
    }

    async fn add_member(&self, membership: &UserWorkspace) -> Result<UserWorkspace> {
        sqlx::query(
            r#"
            INSERT INTO user_workspaces (id, user_id, workspace_id, role, joined_at)
            VALUES ($1, $2, $3, $4, $5)
            "#,
        )
        .bind(membership.id)
        .bind(&membership.user_id)
        .bind(membership.workspace_id)
        .bind(&membership.role)
        .bind(Utc::now())
        .execute(&self.pool)
        .await
        .map_err(|e| ApiError::internal(e.to_string()))?;

        self.get_membership(membership.workspace_id, &membership.user_id)
            .await?
            .ok_or_else(|| ApiError::internal("Failed to retrieve created membership"))
    }

    async fn remove_member(&self, workspace_id: Uuid, user_id: &str) -> Result<()> {
        sqlx::query("DELETE FROM user_workspaces WHERE workspace_id = $1 AND user_id = $2")
            .bind(workspace_id)
            .bind(user_id)
            .execute(&self.pool)
            .await
            .map_err(|e| ApiError::internal(e.to_string()))?;

        Ok(())
    }

    async fn get_membership(&self, workspace_id: Uuid, user_id: &str) -> Result<Option<UserWorkspace>> {
        let row = sqlx::query(
            "SELECT * FROM user_workspaces WHERE workspace_id = $1 AND user_id = $2",
        )
        .bind(workspace_id)
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| ApiError::internal(e.to_string()))?;

        match row {
            Some(r) => Ok(Some(Self::map_membership_row(&r)?)),
            None => Ok(None),
        }
    }

    async fn list_members(&self, workspace_id: Uuid) -> Result<Vec<UserWorkspace>> {
        let rows = sqlx::query("SELECT * FROM user_workspaces WHERE workspace_id = $1 ORDER BY joined_at")
            .bind(workspace_id)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| ApiError::internal(e.to_string()))?;

        Ok(rows.iter().filter_map(|r| Self::map_membership_row(r).ok()).collect())
    }

    async fn update_member_role(&self, workspace_id: Uuid, user_id: &str, role: &str) -> Result<()> {
        sqlx::query(
            "UPDATE user_workspaces SET role = $3 WHERE workspace_id = $1 AND user_id = $2",
        )
        .bind(workspace_id)
        .bind(user_id)
        .bind(role)
        .execute(&self.pool)
        .await
        .map_err(|e| ApiError::internal(e.to_string()))?;

        Ok(())
    }

    async fn user_has_access(&self, workspace_id: Uuid, user_id: &str) -> Result<bool> {
        let row = sqlx::query(
            "SELECT EXISTS(SELECT 1 FROM user_workspaces WHERE workspace_id = $1 AND user_id = $2) as exists",
        )
        .bind(workspace_id)
        .bind(user_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| ApiError::internal(e.to_string()))?;

        Ok(row.try_get::<bool, _>("exists").unwrap_or(false))
    }
}
