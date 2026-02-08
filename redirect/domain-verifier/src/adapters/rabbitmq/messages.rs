use serde::{Deserialize, Serialize};

use crate::model::{VerificationReason, VerificationStatus};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainStateChangedMessage {
    pub domain_id: String,
    pub domain_name: String,
    pub owner_id: String,
    pub status: VerificationStatus,
    pub verification_reason: VerificationReason,
    pub last_check_at: Option<i64>,
    pub next_check_at: Option<i64>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_domain_state_changed_message_serde_roundtrip() {
        let msg = DomainStateChangedMessage {
            domain_id: "d1".into(),
            domain_name: "example.com".into(),
            owner_id: "owner1".into(),
            status: VerificationStatus::Verified,
            verification_reason: VerificationReason::TxtRecordValid,
            last_check_at: Some(1700000000000),
            next_check_at: Some(1700001800000),
        };

        let json = serde_json::to_string(&msg).unwrap();
        let deserialized: DomainStateChangedMessage = serde_json::from_str(&json).unwrap();

        assert_eq!(msg.domain_id, deserialized.domain_id);
        assert_eq!(msg.domain_name, deserialized.domain_name);
        assert_eq!(msg.owner_id, deserialized.owner_id);
        assert_eq!(msg.status, deserialized.status);
        assert_eq!(msg.verification_reason, deserialized.verification_reason);
        assert_eq!(msg.last_check_at, deserialized.last_check_at);
        assert_eq!(msg.next_check_at, deserialized.next_check_at);
    }

    #[test]
    fn test_domain_state_changed_message_with_none_timestamps() {
        let msg = DomainStateChangedMessage {
            domain_id: "d2".into(),
            domain_name: "test.com".into(),
            owner_id: "o2".into(),
            status: VerificationStatus::Pending,
            verification_reason: VerificationReason::NotChecked,
            last_check_at: None,
            next_check_at: None,
        };

        let json = serde_json::to_string(&msg).unwrap();
        let deserialized: DomainStateChangedMessage = serde_json::from_str(&json).unwrap();

        assert!(deserialized.last_check_at.is_none());
        assert!(deserialized.next_check_at.is_none());
    }

    #[test]
    fn test_domain_state_changed_message_failed_status() {
        let msg = DomainStateChangedMessage {
            domain_id: "d3".into(),
            domain_name: "bad.com".into(),
            owner_id: "o3".into(),
            status: VerificationStatus::Failed,
            verification_reason: VerificationReason::ARecordInvalid,
            last_check_at: Some(1700000000000),
            next_check_at: Some(1700000300000),
        };

        let json = serde_json::to_string(&msg).unwrap();
        assert!(json.contains("\"failed\""));
        assert!(json.contains("\"a_record_invalid\""));
    }

    #[test]
    fn test_domain_state_changed_message_dns_error_reason() {
        let msg = DomainStateChangedMessage {
            domain_id: "d4".into(),
            domain_name: "err.com".into(),
            owner_id: "o4".into(),
            status: VerificationStatus::Failed,
            verification_reason: VerificationReason::DnsError("SERVFAIL".into()),
            last_check_at: None,
            next_check_at: None,
        };

        let json = serde_json::to_string(&msg).unwrap();
        let deserialized: DomainStateChangedMessage = serde_json::from_str(&json).unwrap();

        assert_eq!(
            deserialized.verification_reason,
            VerificationReason::DnsError("SERVFAIL".into())
        );
    }

    #[test]
    fn test_domain_state_changed_message_clone() {
        let msg = DomainStateChangedMessage {
            domain_id: "d1".into(),
            domain_name: "example.com".into(),
            owner_id: "o1".into(),
            status: VerificationStatus::Verified,
            verification_reason: VerificationReason::TxtRecordValid,
            last_check_at: Some(100),
            next_check_at: Some(200),
        };

        let cloned = msg.clone();
        assert_eq!(msg.domain_id, cloned.domain_id);
        assert_eq!(msg.status, cloned.status);
    }
}
