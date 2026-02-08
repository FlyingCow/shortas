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
