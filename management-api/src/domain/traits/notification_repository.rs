//! Notification repository trait.

use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::entities::{Notification, Result};

/// Repository for in-app notifications.
#[async_trait]
pub trait NotificationRepository: Send + Sync {
    /// Create a new notification.
    async fn create(&self, notification: &Notification) -> Result<Notification>;

    /// Get a notification by ID.
    async fn get_by_id(&self, id: Uuid) -> Result<Option<Notification>>;

    /// List notifications for a user, newest first.
    async fn list_by_user(&self, user_id: &str, limit: i32) -> Result<Vec<Notification>>;

    /// Mark a notification as read.
    async fn mark_read(&self, id: Uuid) -> Result<()>;

    /// Mark a notification as actioned (e.g. invite accepted/declined).
    async fn mark_actioned(&self, id: Uuid) -> Result<()>;

    /// Mark as actioned any notification whose payload carries this
    /// `invitation_id` (e.g. once the invitation itself has been
    /// accepted/declined, so it stops reappearing in the bell).
    async fn mark_actioned_by_invitation(&self, invitation_id: Uuid) -> Result<()>;
}
