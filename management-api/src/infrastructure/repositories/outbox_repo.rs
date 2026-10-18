//! Outbox repository implementation using SQLx.

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{PgPool, Row};
use uuid::Uuid;

use crate::domain::entities::{ApiError, OutboxMessage, OutboxMessageType, OutboxStatus, Result};
use crate::domain::traits::OutboxRepository;

/// PostgreSQL outbox repository.
pub struct PgOutboxRepository {
    pool: PgPool,
}

impl PgOutboxRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    fn map_row(row: &sqlx::postgres::PgRow) -> Result<OutboxMessage> {
        let id: Uuid = row.try_get("id").map_err(|e| ApiError::internal(e.to_string()))?;
        let message_type_str: String = row.try_get("message_type").unwrap_or_default();
        let payload_str: String = row.try_get("payload").unwrap_or_else(|_| "{}".to_string());
        let status_str: String = row.try_get("status").unwrap_or_else(|_| "Pending".to_string());
        let retry_count: i32 = row.try_get("retry_count").unwrap_or(0);
        let max_retries: i32 = row.try_get("max_retries").unwrap_or(3);
        let error_message: Option<String> = row.try_get("error_message").ok();
        let created_at: DateTime<Utc> = row.try_get("created_at").unwrap_or_else(|_| Utc::now());
        let processed_at: Option<DateTime<Utc>> = row.try_get("processed_at").ok();
        let next_retry_at: Option<DateTime<Utc>> = row.try_get("next_retry_at").ok();

        let message_type = OutboxMessageType::from_str(&message_type_str)
            .unwrap_or(OutboxMessageType::IndexRoute);

        let payload: serde_json::Value = serde_json::from_str(&payload_str)
            .unwrap_or(serde_json::json!({}));

        let status = OutboxStatus::from_str(&status_str);

        Ok(OutboxMessage {
            id,
            message_type,
            payload,
            status,
            retry_count,
            max_retries,
            error_message,
            created_at,
            processed_at,
            next_retry_at,
        })
    }
}

#[async_trait]
impl OutboxRepository for PgOutboxRepository {
    async fn create(&self, message: &OutboxMessage) -> Result<OutboxMessage> {
        let payload_str = serde_json::to_string(&message.payload).unwrap_or_else(|_| "{}".to_string());

        sqlx::query(
            r#"
            INSERT INTO outbox_messages (id, message_type, payload, status, retry_count, max_retries, created_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#,
        )
        .bind(message.id)
        .bind(message.message_type.as_str())
        .bind(&payload_str)
        .bind(message.status.as_str())
        .bind(message.retry_count)
        .bind(message.max_retries)
        .bind(Utc::now())
        .execute(&self.pool)
        .await
        .map_err(|e| ApiError::internal(e.to_string()))?;

        self.get_by_id(message.id)
            .await?
            .ok_or_else(|| ApiError::internal("Failed to retrieve created outbox message"))
    }

    async fn get_pending(&self, limit: i32) -> Result<Vec<OutboxMessage>> {
        let rows = sqlx::query(
            r#"
            SELECT * FROM outbox_messages
            WHERE status = 'Pending'
              AND (next_retry_at IS NULL OR next_retry_at <= NOW())
            ORDER BY created_at
            LIMIT $1
            "#,
        )
        .bind(limit)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| ApiError::internal(e.to_string()))?;

        Ok(rows.iter().filter_map(|r| Self::map_row(r).ok()).collect())
    }

    async fn mark_processing(&self, id: Uuid) -> Result<()> {
        sqlx::query("UPDATE outbox_messages SET status = 'Processing' WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| ApiError::internal(e.to_string()))?;

        Ok(())
    }

    async fn mark_completed(&self, id: Uuid) -> Result<()> {
        sqlx::query(
            "UPDATE outbox_messages SET status = 'Completed', processed_at = NOW() WHERE id = $1",
        )
        .bind(id)
        .execute(&self.pool)
        .await
        .map_err(|e| ApiError::internal(e.to_string()))?;

        Ok(())
    }

    async fn mark_failed(&self, id: Uuid, error: &str) -> Result<()> {
        // Get current message to check retry count
        let message = self.get_by_id(id).await?;

        if let Some(msg) = message {
            let new_retry_count = msg.retry_count + 1;
            let new_status = if new_retry_count >= msg.max_retries {
                "Failed"
            } else {
                "Pending"
            };

            // Exponential backoff
            let delay_seconds = 2_i64.pow(new_retry_count as u32);
            let next_retry = Utc::now() + chrono::Duration::seconds(delay_seconds);

            sqlx::query(
                r#"
                UPDATE outbox_messages
                SET status = $2, retry_count = $3, error_message = $4, next_retry_at = $5
                WHERE id = $1
                "#,
            )
            .bind(id)
            .bind(new_status)
            .bind(new_retry_count)
            .bind(error)
            .bind(next_retry)
            .execute(&self.pool)
            .await
            .map_err(|e| ApiError::internal(e.to_string()))?;
        }

        Ok(())
    }

    async fn cleanup_completed(&self, older_than_days: i32) -> Result<i64> {
        let result = sqlx::query(
            r#"
            DELETE FROM outbox_messages
            WHERE status = 'Completed'
              AND processed_at < NOW() - ($1 || ' days')::interval
            "#,
        )
        .bind(older_than_days)
        .execute(&self.pool)
        .await
        .map_err(|e| ApiError::internal(e.to_string()))?;

        Ok(result.rows_affected() as i64)
    }

    async fn get_by_id(&self, id: Uuid) -> Result<Option<OutboxMessage>> {
        let row = sqlx::query("SELECT * FROM outbox_messages WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| ApiError::internal(e.to_string()))?;

        match row {
            Some(r) => Ok(Some(Self::map_row(&r)?)),
            None => Ok(None),
        }
    }
}
