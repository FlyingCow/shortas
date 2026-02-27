use anyhow::Result;

/// Represents an ACME HTTP-01 challenge
#[derive(Clone, Debug)]
pub struct Challenge {
    pub domain: String,
    pub token: String,
    pub key_authorization: String,
}

/// Trait for retrieving ACME HTTP-01 challenges
#[async_trait::async_trait()]
pub trait ChallengeStore {
    async fn get_challenge(&self, domain: &str, token: &str) -> Result<Option<Challenge>>;
}

/// Trait for caching ACME challenges
#[async_trait::async_trait()]
pub trait ChallengeCache {
    async fn get_challenge(&self, domain: &str, token: &str) -> Result<Option<Challenge>>;
    async fn invalidate(&self, domain: &str, token: &str) -> Result<()>;
}
