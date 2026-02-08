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
