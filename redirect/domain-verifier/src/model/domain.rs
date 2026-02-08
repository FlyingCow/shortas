use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "snake_case")]
pub enum VerificationStatus {
    #[default]
    Pending,
    Verified,
    Failed,
}

impl std::fmt::Display for VerificationStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VerificationStatus::Pending => write!(f, "pending"),
            VerificationStatus::Verified => write!(f, "verified"),
            VerificationStatus::Failed => write!(f, "failed"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "snake_case")]
pub enum VerificationReason {
    #[default]
    NotChecked,
    TxtRecordValid,
    TxtRecordMissing,
    TxtRecordMismatch,
    ARecordValid,
    ARecordInvalid,
    ARecordMissing,
    AaaaRecordInvalid,
    DnsTimeout,
    #[serde(rename = "dns_error")]
    DnsError(String),
}

impl std::fmt::Display for VerificationReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VerificationReason::NotChecked => write!(f, "not_checked"),
            VerificationReason::TxtRecordValid => write!(f, "txt_record_valid"),
            VerificationReason::TxtRecordMissing => write!(f, "txt_record_missing"),
            VerificationReason::TxtRecordMismatch => write!(f, "txt_record_mismatch"),
            VerificationReason::ARecordValid => write!(f, "a_record_valid"),
            VerificationReason::ARecordInvalid => write!(f, "a_record_invalid"),
            VerificationReason::ARecordMissing => write!(f, "a_record_missing"),
            VerificationReason::AaaaRecordInvalid => write!(f, "aaaa_record_invalid"),
            VerificationReason::DnsTimeout => write!(f, "dns_timeout"),
            VerificationReason::DnsError(msg) => write!(f, "dns_error: {}", msg),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Domain {
    pub id: String,
    pub name: String,
    pub owner_id: String,
    pub status: VerificationStatus,
    pub verification_reason: VerificationReason,
    #[serde(with = "chrono::serde::ts_milliseconds_option")]
    pub last_check_at: Option<DateTime<Utc>>,
    #[serde(with = "chrono::serde::ts_milliseconds_option")]
    pub next_check_at: Option<DateTime<Utc>>,
    #[serde(with = "chrono::serde::ts_milliseconds")]
    pub created_at: DateTime<Utc>,
}

impl Domain {
    pub fn new(id: String, name: String, owner_id: String) -> Self {
        Self {
            id,
            name,
            owner_id,
            status: VerificationStatus::Pending,
            verification_reason: VerificationReason::NotChecked,
            last_check_at: None,
            next_check_at: Some(Utc::now()),
            created_at: Utc::now(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_domain_new_sets_defaults() {
        let domain = Domain::new("d1".into(), "example.com".into(), "owner1".into());

        assert_eq!(domain.id, "d1");
        assert_eq!(domain.name, "example.com");
        assert_eq!(domain.owner_id, "owner1");
        assert_eq!(domain.status, VerificationStatus::Pending);
        assert_eq!(domain.verification_reason, VerificationReason::NotChecked);
        assert!(domain.last_check_at.is_none());
        assert!(domain.next_check_at.is_some());
    }

    #[test]
    fn test_verification_status_default() {
        let status: VerificationStatus = Default::default();
        assert_eq!(status, VerificationStatus::Pending);
    }

    #[test]
    fn test_verification_reason_default() {
        let reason: VerificationReason = Default::default();
        assert_eq!(reason, VerificationReason::NotChecked);
    }

    #[test]
    fn test_verification_status_display() {
        assert_eq!(VerificationStatus::Pending.to_string(), "pending");
        assert_eq!(VerificationStatus::Verified.to_string(), "verified");
        assert_eq!(VerificationStatus::Failed.to_string(), "failed");
    }

    #[test]
    fn test_verification_reason_display() {
        assert_eq!(VerificationReason::NotChecked.to_string(), "not_checked");
        assert_eq!(VerificationReason::TxtRecordValid.to_string(), "txt_record_valid");
        assert_eq!(VerificationReason::TxtRecordMissing.to_string(), "txt_record_missing");
        assert_eq!(VerificationReason::TxtRecordMismatch.to_string(), "txt_record_mismatch");
        assert_eq!(VerificationReason::ARecordValid.to_string(), "a_record_valid");
        assert_eq!(VerificationReason::ARecordInvalid.to_string(), "a_record_invalid");
        assert_eq!(VerificationReason::ARecordMissing.to_string(), "a_record_missing");
        assert_eq!(VerificationReason::AaaaRecordInvalid.to_string(), "aaaa_record_invalid");
        assert_eq!(VerificationReason::DnsTimeout.to_string(), "dns_timeout");
        assert_eq!(
            VerificationReason::DnsError("lookup failed".into()).to_string(),
            "dns_error: lookup failed"
        );
    }

    #[test]
    fn test_verification_status_serde_roundtrip() {
        let statuses = vec![
            VerificationStatus::Pending,
            VerificationStatus::Verified,
            VerificationStatus::Failed,
        ];
        for status in statuses {
            let json = serde_json::to_string(&status).unwrap();
            let deserialized: VerificationStatus = serde_json::from_str(&json).unwrap();
            assert_eq!(status, deserialized);
        }
    }

    #[test]
    fn test_verification_status_serde_snake_case() {
        let json = serde_json::to_string(&VerificationStatus::Pending).unwrap();
        assert_eq!(json, r#""pending""#);

        let json = serde_json::to_string(&VerificationStatus::Verified).unwrap();
        assert_eq!(json, r#""verified""#);

        let json = serde_json::to_string(&VerificationStatus::Failed).unwrap();
        assert_eq!(json, r#""failed""#);
    }

    #[test]
    fn test_verification_reason_serde_roundtrip() {
        let reasons = vec![
            VerificationReason::NotChecked,
            VerificationReason::TxtRecordValid,
            VerificationReason::TxtRecordMissing,
            VerificationReason::TxtRecordMismatch,
            VerificationReason::ARecordValid,
            VerificationReason::ARecordInvalid,
            VerificationReason::ARecordMissing,
            VerificationReason::AaaaRecordInvalid,
            VerificationReason::DnsTimeout,
            VerificationReason::DnsError("test error".into()),
        ];
        for reason in reasons {
            let json = serde_json::to_string(&reason).unwrap();
            let deserialized: VerificationReason = serde_json::from_str(&json).unwrap();
            assert_eq!(reason, deserialized);
        }
    }

    #[test]
    fn test_domain_serde_roundtrip() {
        let domain = Domain::new("d1".into(), "example.com".into(), "owner1".into());
        let json = serde_json::to_string(&domain).unwrap();
        let deserialized: Domain = serde_json::from_str(&json).unwrap();

        assert_eq!(domain.id, deserialized.id);
        assert_eq!(domain.name, deserialized.name);
        assert_eq!(domain.owner_id, deserialized.owner_id);
        assert_eq!(domain.status, deserialized.status);
        assert_eq!(domain.verification_reason, deserialized.verification_reason);
    }

    #[test]
    fn test_domain_new_timestamps_are_recent() {
        let before = Utc::now();
        let domain = Domain::new("d1".into(), "test.com".into(), "o1".into());
        let after = Utc::now();

        assert!(domain.created_at >= before && domain.created_at <= after);
        let next = domain.next_check_at.unwrap();
        assert!(next >= before && next <= after);
    }
}
