use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum OrderStatus {
    Pending,
    ChallengeCreated,
    ChallengeReady,
    Processing,
    Valid,
    Failed,
    Expired,
}

impl Default for OrderStatus {
    fn default() -> Self {
        Self::Pending
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CertificateOrder {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<mongodb::bson::oid::ObjectId>,
    pub order_id: String,
    pub domain: String,
    pub owner_id: String,
    pub status: OrderStatus,
    pub error_message: Option<String>,
    pub retry_count: u32,
    pub max_retries: u32,

    // ACME protocol fields
    pub acme_order_url: Option<String>,
    pub acme_authorization_url: Option<String>,
    pub acme_finalize_url: Option<String>,
    pub acme_certificate_url: Option<String>,

    // Timing
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
    pub next_retry_at: Option<DateTime<Utc>>,
}

impl CertificateOrder {
    pub fn new(domain: String, owner_id: String, max_retries: u32) -> Self {
        let now = Utc::now();
        Self {
            id: None,
            order_id: uuid::Uuid::new_v4().to_string(),
            domain,
            owner_id,
            status: OrderStatus::Pending,
            error_message: None,
            retry_count: 0,
            max_retries,
            acme_order_url: None,
            acme_authorization_url: None,
            acme_finalize_url: None,
            acme_certificate_url: None,
            created_at: now,
            updated_at: now,
            expires_at: None,
            next_retry_at: None,
        }
    }
}
