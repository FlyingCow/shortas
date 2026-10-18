//! Outbox message entity for transactional messaging.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Outbox message status.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OutboxStatus {
    Pending,
    Processing,
    Completed,
    Failed,
}

impl OutboxStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            OutboxStatus::Pending => "Pending",
            OutboxStatus::Processing => "Processing",
            OutboxStatus::Completed => "Completed",
            OutboxStatus::Failed => "Failed",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "Processing" => OutboxStatus::Processing,
            "Completed" => OutboxStatus::Completed,
            "Failed" => OutboxStatus::Failed,
            _ => OutboxStatus::Pending,
        }
    }
}

/// Outbox message types.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OutboxMessageType {
    /// Index route in Elasticsearch.
    IndexRoute,
    /// Remove route from Elasticsearch.
    DeleteRouteIndex,
    /// Verify domain DNS.
    VerifyDomain,
    /// Check route against Safe Browsing.
    CheckRouteSafety,
    /// Send notification.
    SendNotification,
}

impl OutboxMessageType {
    pub fn as_str(&self) -> &'static str {
        match self {
            OutboxMessageType::IndexRoute => "IndexRoute",
            OutboxMessageType::DeleteRouteIndex => "DeleteRouteIndex",
            OutboxMessageType::VerifyDomain => "VerifyDomain",
            OutboxMessageType::CheckRouteSafety => "CheckRouteSafety",
            OutboxMessageType::SendNotification => "SendNotification",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "IndexRoute" => Some(OutboxMessageType::IndexRoute),
            "DeleteRouteIndex" => Some(OutboxMessageType::DeleteRouteIndex),
            "VerifyDomain" => Some(OutboxMessageType::VerifyDomain),
            "CheckRouteSafety" => Some(OutboxMessageType::CheckRouteSafety),
            "SendNotification" => Some(OutboxMessageType::SendNotification),
            _ => None,
        }
    }
}

/// Outbox message entity.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutboxMessage {
    pub id: Uuid,
    pub message_type: OutboxMessageType,
    pub payload: serde_json::Value,
    pub status: OutboxStatus,
    pub retry_count: i32,
    pub max_retries: i32,
    pub error_message: Option<String>,
    pub created_at: DateTime<Utc>,
    pub processed_at: Option<DateTime<Utc>>,
    pub next_retry_at: Option<DateTime<Utc>>,
}

impl OutboxMessage {
    /// Create a new outbox message.
    pub fn new(message_type: OutboxMessageType, payload: serde_json::Value) -> Self {
        Self {
            id: Uuid::new_v4(),
            message_type,
            payload,
            status: OutboxStatus::Pending,
            retry_count: 0,
            max_retries: 3,
            error_message: None,
            created_at: Utc::now(),
            processed_at: None,
            next_retry_at: None,
        }
    }

    /// Create a message to index a route.
    pub fn index_route(route_id: Uuid) -> Self {
        Self::new(
            OutboxMessageType::IndexRoute,
            serde_json::json!({ "route_id": route_id }),
        )
    }

    /// Create a message to delete a route index.
    pub fn delete_route_index(route_id: Uuid) -> Self {
        Self::new(
            OutboxMessageType::DeleteRouteIndex,
            serde_json::json!({ "route_id": route_id }),
        )
    }

    /// Create a message to verify a domain.
    pub fn verify_domain(domain_id: Uuid) -> Self {
        Self::new(
            OutboxMessageType::VerifyDomain,
            serde_json::json!({ "domain_id": domain_id }),
        )
    }

    /// Create a message to check route safety.
    pub fn check_route_safety(route_id: Uuid, url: String) -> Self {
        Self::new(
            OutboxMessageType::CheckRouteSafety,
            serde_json::json!({ "route_id": route_id, "url": url }),
        )
    }

    /// Mark the message as processing.
    pub fn mark_processing(&mut self) {
        self.status = OutboxStatus::Processing;
    }

    /// Mark the message as completed.
    pub fn mark_completed(&mut self) {
        self.status = OutboxStatus::Completed;
        self.processed_at = Some(Utc::now());
    }

    /// Mark the message as failed with retry.
    pub fn mark_failed(&mut self, error: String) {
        self.retry_count += 1;
        self.error_message = Some(error);

        if self.retry_count >= self.max_retries {
            self.status = OutboxStatus::Failed;
        } else {
            self.status = OutboxStatus::Pending;
            // Exponential backoff: 2^retry_count seconds
            let delay = chrono::Duration::seconds(2_i64.pow(self.retry_count as u32));
            self.next_retry_at = Some(Utc::now() + delay);
        }
    }

    /// Check if message can be retried.
    pub fn can_retry(&self) -> bool {
        self.retry_count < self.max_retries
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_outbox_message_creation() {
        let msg = OutboxMessage::index_route(Uuid::new_v4());
        assert!(matches!(msg.message_type, OutboxMessageType::IndexRoute));
        assert!(matches!(msg.status, OutboxStatus::Pending));
        assert_eq!(msg.retry_count, 0);
    }

    #[test]
    fn test_outbox_message_retry() {
        let mut msg = OutboxMessage::index_route(Uuid::new_v4());

        msg.mark_failed("Error 1".to_string());
        assert!(msg.can_retry());
        assert_eq!(msg.retry_count, 1);

        msg.mark_failed("Error 2".to_string());
        msg.mark_failed("Error 3".to_string());
        assert!(!msg.can_retry());
        assert!(matches!(msg.status, OutboxStatus::Failed));
    }
}
