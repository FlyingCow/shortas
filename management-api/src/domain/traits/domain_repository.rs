//! Domain repository trait.

use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::entities::{Result, RouteDomain};

/// Domain repository trait for database operations.
#[async_trait]
pub trait DomainRepository: Send + Sync {
    /// Get domain by ID.
    async fn get_by_id(&self, id: Uuid) -> Result<Option<RouteDomain>>;

    /// Get domains by multiple IDs.
    async fn get_by_ids(&self, ids: &[Uuid]) -> Result<Vec<RouteDomain>>;

    /// Get domain by name.
    async fn get_by_name(&self, name: &str) -> Result<Option<RouteDomain>>;

    /// List domains for an owner.
    async fn list_by_owner(&self, owner_id: &str) -> Result<Vec<RouteDomain>>;

    /// List shared domains.
    async fn list_shared(&self) -> Result<Vec<RouteDomain>>;

    /// List domains accessible by user (owned + shared).
    async fn list_accessible(&self, user_id: &str) -> Result<Vec<RouteDomain>>;

    /// Create a new domain.
    async fn create(&self, domain: &RouteDomain) -> Result<RouteDomain>;

    /// Update an existing domain.
    async fn update(&self, domain: &RouteDomain) -> Result<RouteDomain>;

    /// Delete a domain by ID.
    async fn delete(&self, id: Uuid) -> Result<()>;

    /// Check if domain name exists.
    async fn name_exists(&self, name: &str) -> Result<bool>;

    /// Get domains pending verification.
    async fn get_pending_verification(&self, limit: i32) -> Result<Vec<RouteDomain>>;

    /// Update verification status.
    async fn update_verification_status(
        &self,
        id: Uuid,
        status: &str,
        reason: &str,
    ) -> Result<()>;
}
