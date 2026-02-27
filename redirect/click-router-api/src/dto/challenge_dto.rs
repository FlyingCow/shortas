use chrono::{DateTime, Utc};
use salvo::oapi::ToSchema;
use serde::{Deserialize, Serialize};

use crate::model::challenge::Challenge;

/// ACME HTTP-01 challenge DTO for API requests/responses
#[derive(Clone, Serialize, Deserialize, Debug, ToSchema)]
pub struct ChallengeDto {
    /// Domain name the challenge is for
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// Challenge token (used in URL path)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    /// Key authorization (response to serve at /.well-known/acme-challenge/{token})
    pub key_authorization: String,
    /// When the challenge expires (defaults to 1 hour from creation if not specified)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<DateTime<Utc>>,
}

impl ChallengeDto {
    pub fn new(key_authorization: impl Into<String>) -> Self {
        Self {
            domain: None,
            token: None,
            key_authorization: key_authorization.into(),
            expires_at: None,
        }
    }

    pub fn with_expiry(mut self, expires_at: DateTime<Utc>) -> Self {
        self.expires_at = Some(expires_at);
        self
    }
}

impl From<Challenge> for ChallengeDto {
    fn from(challenge: Challenge) -> Self {
        Self {
            domain: Some(challenge.domain),
            token: Some(challenge.token),
            key_authorization: challenge.key_authorization,
            expires_at: Some(challenge.expires_at),
        }
    }
}

impl From<&Challenge> for ChallengeDto {
    fn from(challenge: &Challenge) -> Self {
        Self {
            domain: Some(challenge.domain.clone()),
            token: Some(challenge.token.clone()),
            key_authorization: challenge.key_authorization.clone(),
            expires_at: Some(challenge.expires_at),
        }
    }
}
