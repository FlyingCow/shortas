//! Outbox repository trait.

use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::entities::{OutboxMessage, Result};

/// Outbox repository trait for database operations.
#[async_trait]
pub trait OutboxRepository: Send + Sync {
    /// Create a new outbox message.
    async fn create(&self, message: &OutboxMessage) -> Result<OutboxMessage>;

    /// Get pending messages ready for processing.
    async fn get_pending(&self, limit: i32) -> Result<Vec<OutboxMessage>>;

    /// Mark message as processing.
    async fn mark_processing(&self, id: Uuid) -> Result<()>;

    /// Mark message as completed.
    async fn mark_completed(&self, id: Uuid) -> Result<()>;

    /// Mark message as failed.
    async fn mark_failed(&self, id: Uuid, error: &str) -> Result<()>;

    /// Delete old completed messages.
    async fn cleanup_completed(&self, older_than_days: i32) -> Result<i64>;

    /// Get message by ID.
    async fn get_by_id(&self, id: Uuid) -> Result<Option<OutboxMessage>>;
}
