//! Generic notification event: the contract published to RabbitMQ and
//! consumed by the `notification-worker` service to render and deliver
//! a notification (in-app and/or email) from a template stored in S3.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Who the notification is for and where to deliver it.
/// Split from `context` because it's routing information, not render data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationRecipient {
    /// Local user id, when known. Required for the `in_app` channel.
    pub user_id: Option<String>,
    /// Explicit email address. Falls back to a `UserRepository` lookup by
    /// `user_id` when absent (e.g. an invite already knows the email before
    /// the invitee has an account).
    pub email: Option<String>,
}

/// A notification to render (from its S3 template set) and deliver.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationEvent {
    /// Template key, e.g. "workspace.invite_created".
    pub notification_id: String,
    pub recipient: NotificationRecipient,
    /// Handlebars render context.
    pub context: serde_json::Value,
    /// Dedupe/trace key, carried through from the outbox message id.
    pub idempotency_key: Uuid,
    pub occurred_at: DateTime<Utc>,
}

impl NotificationEvent {
    pub fn new(
        notification_id: impl Into<String>,
        recipient: NotificationRecipient,
        context: serde_json::Value,
        idempotency_key: Uuid,
    ) -> Self {
        Self {
            notification_id: notification_id.into(),
            recipient,
            context,
            idempotency_key,
            occurred_at: Utc::now(),
        }
    }
}
