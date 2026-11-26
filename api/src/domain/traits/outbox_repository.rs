//! Outbox repository trait.

use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::entities::{OutboxMessage, Result};

/// Outbox repository trait for database operations.
#[cfg_attr(any(test, feature = "mocks"), mockall::automock)]
#[async_trait]
pub trait OutboxRepository: Send + Sync {
    /// Create a new outbox message.
    async fn create(&self, message: &OutboxMessage) -> Result<OutboxMessage>;

    /// Create the message unless an equal one (same type and payload) is
    /// still pending. Returns whether it was created.
    async fn create_unless_pending(&self, message: &OutboxMessage) -> Result<bool>;

    /// Get pending messages ready for processing.
    async fn get_pending(&self, limit: i32) -> Result<Vec<OutboxMessage>>;

    /// Return conversion messages left unfinished by a restart to the queue.
    /// Only those are safe to send twice.
    async fn requeue_interrupted(&self) -> Result<i64>;

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
