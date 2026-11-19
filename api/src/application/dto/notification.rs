//! Notification DTOs for API responses.

use salvo::oapi::ToSchema;
use serde::{Deserialize, Serialize};

use crate::domain::entities::Notification;

/// Notification response DTO.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct NotificationDto {
    pub id: String,
    #[serde(rename = "type")]
    pub notification_type: String,
    pub payload: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rendered_text: Option<String>,
    pub status: String,
    pub created_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_at: Option<String>,
}

impl NotificationDto {
    pub fn from_entity(notification: Notification) -> Self {
        Self {
            id: notification.id.to_string(),
            notification_type: notification.notification_type,
            payload: notification.payload,
            rendered_text: notification.rendered_text,
            status: notification.status.as_str().to_string(),
            created_at: notification.created_at.to_rfc3339(),
            read_at: notification.read_at.map(|dt| dt.to_rfc3339()),
        }
    }
}
