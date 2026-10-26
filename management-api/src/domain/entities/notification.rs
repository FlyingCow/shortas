//! In-app notification center entity.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Notification status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NotificationStatus {
    Unread,
    Read,
    /// The user has acted on it (e.g. accepted/declined an invite).
    Actioned,
}

impl NotificationStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            NotificationStatus::Unread => "Unread",
            NotificationStatus::Read => "Read",
            NotificationStatus::Actioned => "Actioned",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "Read" => NotificationStatus::Read,
            "Actioned" => NotificationStatus::Actioned,
            _ => NotificationStatus::Unread,
        }
    }
}

/// A notification delivered to a user's in-app notification center.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Notification {
    pub id: Uuid,
    pub user_id: String,
    /// e.g. "WorkspaceInvite", "InviteAccepted", "InviteDeclined".
    pub notification_type: String,
    pub payload: serde_json::Value,
    /// Human-readable text rendered from the notification's `in_app` template.
    /// `None` for notifications created before template rendering existed.
    pub rendered_text: Option<String>,
    pub status: NotificationStatus,
    pub created_at: DateTime<Utc>,
    pub read_at: Option<DateTime<Utc>>,
}

impl Notification {
    pub fn new(user_id: String, notification_type: String, payload: serde_json::Value) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id,
            notification_type,
            payload,
            rendered_text: None,
            status: NotificationStatus::Unread,
            created_at: Utc::now(),
            read_at: None,
        }
    }

    /// Create a notification with pre-rendered display text (the normal path
    /// for notifications produced by the template-rendering pipeline).
    pub fn with_rendered_text(
        user_id: String,
        notification_type: String,
        payload: serde_json::Value,
        rendered_text: String,
    ) -> Self {
        Self {
            rendered_text: Some(rendered_text),
            ..Self::new(user_id, notification_type, payload)
        }
    }
}
