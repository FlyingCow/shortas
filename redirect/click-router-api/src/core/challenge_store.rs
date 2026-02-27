use anyhow::Result;
use chrono::{DateTime, Utc};
use dyn_clone::{clone_trait_object, DynClone};

use crate::model::challenge::Challenge;

/// Trait for storing and retrieving ACME HTTP-01 challenges
#[async_trait::async_trait()]
pub trait ChallengeStore: DynClone {
    /// Store an ACME challenge for a domain
    async fn store_challenge(
        &self,
        domain: &str,
        token: &str,
        key_authorization: &str,
        expires_at: DateTime<Utc>,
    ) -> Result<()>;

    /// Get an ACME challenge by domain and token
    async fn get_challenge(&self, domain: &str, token: &str) -> Result<Option<Challenge>>;

    /// Delete a specific challenge
    async fn delete_challenge(&self, domain: &str, token: &str) -> Result<()>;

    /// Delete all challenges for a domain
    async fn delete_domain_challenges(&self, domain: &str) -> Result<u64>;

    /// Clean up expired challenges
    async fn cleanup_expired(&self) -> Result<u64>;
}
clone_trait_object!(ChallengeStore);
