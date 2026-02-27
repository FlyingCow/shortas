use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// ACME HTTP-01 challenge model for Let's Encrypt certificate validation
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Challenge {
    /// Domain name the challenge is for
    pub domain: String,
    /// Challenge token (used in URL path)
    pub token: String,
    /// Key authorization (response to serve)
    pub key_authorization: String,
    /// When the challenge expires
    pub expires_at: DateTime<Utc>,
}

impl Challenge {
    pub fn new(
        domain: impl Into<String>,
        token: impl Into<String>,
        key_authorization: impl Into<String>,
        expires_at: DateTime<Utc>,
    ) -> Self {
        Self {
            domain: domain.into(),
            token: token.into(),
            key_authorization: key_authorization.into(),
            expires_at,
        }
    }
}
