use serde::{Deserialize, Serialize};

/// Message published to RabbitMQ when a route's status changes due to safety verification.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteStatusChangedMessage {
    pub route_id: String,
    pub link: String,
    pub owner_id: Option<String>,
    pub workspace_id: Option<String>,
    pub previous_status: String,
    pub new_status: String,
    pub blocked_reason: Option<String>,
    pub threat_type: Option<String>,
    pub threat_url: Option<String>,
    pub checked_at: i64,
    pub next_check_at: Option<i64>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_route_status_changed_message_serde_roundtrip() {
        let msg = RouteStatusChangedMessage {
            route_id: "route123".into(),
            link: "short-link".into(),
            owner_id: Some("owner1".into()),
            workspace_id: Some("ws1".into()),
            previous_status: "Active".into(),
            new_status: "Blocked".into(),
            blocked_reason: Some("Safe Browsing: MALWARE".into()),
            threat_type: Some("MALWARE".into()),
            threat_url: Some("https://malware.example.com".into()),
            checked_at: 1700000000000,
            next_check_at: Some(1700003600000),
        };

        let json = serde_json::to_string(&msg).unwrap();
        let deserialized: RouteStatusChangedMessage = serde_json::from_str(&json).unwrap();

        assert_eq!(msg.route_id, deserialized.route_id);
        assert_eq!(msg.new_status, deserialized.new_status);
        assert_eq!(msg.threat_type, deserialized.threat_type);
    }

    #[test]
    fn test_route_status_changed_message_minimal() {
        let msg = RouteStatusChangedMessage {
            route_id: "route123".into(),
            link: "test".into(),
            owner_id: None,
            workspace_id: None,
            previous_status: "Active".into(),
            new_status: "Blocked".into(),
            blocked_reason: None,
            threat_type: None,
            threat_url: None,
            checked_at: 1700000000000,
            next_check_at: None,
        };

        let json = serde_json::to_string(&msg).unwrap();
        let deserialized: RouteStatusChangedMessage = serde_json::from_str(&json).unwrap();

        assert_eq!(msg.route_id, deserialized.route_id);
        assert!(deserialized.owner_id.is_none());
    }
}
