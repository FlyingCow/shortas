//! Notification repository implementation using SQLx.

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{PgPool, Row};
use uuid::Uuid;

use crate::domain::entities::{ApiError, Notification, NotificationStatus, Result};
use crate::domain::traits::NotificationRepository;

/// PostgreSQL notification repository.
pub struct PgNotificationRepository {
    pool: PgPool,
}

impl PgNotificationRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    fn map_row(row: &sqlx::postgres::PgRow) -> Result<Notification> {
        let id: Uuid = row.try_get("id").map_err(|e| ApiError::internal(e.to_string()))?;
        let user_id: String = row.try_get("user_id").unwrap_or_default();
        let notification_type: String = row.try_get("type").unwrap_or_default();
        let payload: serde_json::Value = row.try_get("payload").unwrap_or_else(|_| serde_json::json!({}));
        let rendered_text: Option<String> = row.try_get("rendered_text").ok();
        let status_str: String = row.try_get("status").unwrap_or_else(|_| "Unread".to_string());
        let created_at: DateTime<Utc> = row.try_get("created_at").unwrap_or_else(|_| Utc::now());
        let read_at: Option<DateTime<Utc>> = row.try_get("read_at").ok();

        Ok(Notification {
            id,
            user_id,
            notification_type,
            payload,
            rendered_text,
            status: NotificationStatus::from_str(&status_str),
            created_at,
            read_at,
        })
    }
}

#[async_trait]
impl NotificationRepository for PgNotificationRepository {
    async fn create(&self, notification: &Notification) -> Result<Notification> {
        sqlx::query(
            r#"
            INSERT INTO notifications (id, user_id, type, payload, rendered_text, status, created_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#,
        )
        .bind(notification.id)
        .bind(&notification.user_id)
        .bind(&notification.notification_type)
        .bind(&notification.payload)
        .bind(&notification.rendered_text)
        .bind(notification.status.as_str())
        .bind(notification.created_at)
        .execute(&self.pool)
        .await
        .map_err(|e| ApiError::internal(e.to_string()))?;

        self.get_by_id(notification.id)
            .await?
            .ok_or_else(|| ApiError::internal("Failed to retrieve created notification"))
    }

    async fn get_by_id(&self, id: Uuid) -> Result<Option<Notification>> {
        let row = sqlx::query("SELECT * FROM notifications WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| ApiError::internal(e.to_string()))?;

        match row {
            Some(r) => Ok(Some(Self::map_row(&r)?)),
            None => Ok(None),
        }
    }

    async fn list_by_user(&self, user_id: &str, limit: i32) -> Result<Vec<Notification>> {
        let rows = sqlx::query(
            "SELECT * FROM notifications WHERE user_id = $1 ORDER BY created_at DESC LIMIT $2",
        )
        .bind(user_id)
        .bind(limit)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| ApiError::internal(e.to_string()))?;

        Ok(rows.iter().filter_map(|r| Self::map_row(r).ok()).collect())
    }

    async fn mark_read(&self, id: Uuid) -> Result<()> {
        sqlx::query(
            "UPDATE notifications SET status = 'Read', read_at = NOW() WHERE id = $1 AND status = 'Unread'",
        )
        .bind(id)
        .execute(&self.pool)
        .await
        .map_err(|e| ApiError::internal(e.to_string()))?;

        Ok(())
    }

    async fn mark_actioned(&self, id: Uuid) -> Result<()> {
        sqlx::query("UPDATE notifications SET status = 'Actioned', read_at = COALESCE(read_at, NOW()) WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| ApiError::internal(e.to_string()))?;

        Ok(())
    }

    async fn mark_actioned_by_invitation(&self, invitation_id: Uuid) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE notifications
            SET status = 'Actioned', read_at = COALESCE(read_at, NOW())
            WHERE status != 'Actioned' AND payload->>'invitation_id' = $1
            "#,
        )
        .bind(invitation_id.to_string())
        .execute(&self.pool)
        .await
        .map_err(|e| ApiError::internal(e.to_string()))?;

        Ok(())
    }
}
