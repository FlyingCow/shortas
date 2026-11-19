use serde::Deserialize;

/// Message received when a domain's verification status changes
#[derive(Debug, Deserialize)]
pub struct DomainStateChangedMessage {
    pub domain_id: String,
    pub domain_name: String,
    pub owner_id: String,
    pub status: String,
    pub verification_reason: Option<String>,
    pub last_check_at: Option<i64>,
    pub next_check_at: Option<i64>,
}
