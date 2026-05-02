//! Certificate repository trait.

use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::entities::{Certificate, Result};

/// Certificate repository trait for database operations.
#[async_trait]
pub trait CertificateRepository: Send + Sync {
    /// Get certificate by ID.
    async fn get_by_id(&self, id: Uuid) -> Result<Option<Certificate>>;

    /// Get certificate by domain ID.
    async fn get_by_domain(&self, domain_id: Uuid) -> Result<Option<Certificate>>;

    /// List certificates for an owner.
    async fn list_by_owner(&self, owner_id: &str) -> Result<Vec<Certificate>>;

    /// Create a new certificate.
    async fn create(&self, certificate: &Certificate) -> Result<Certificate>;

    /// Update an existing certificate.
    async fn update(&self, certificate: &Certificate) -> Result<Certificate>;

    /// Delete a certificate by ID.
    async fn delete(&self, id: Uuid) -> Result<()>;

    /// Delete certificate by domain ID.
    async fn delete_by_domain(&self, domain_id: Uuid) -> Result<()>;

    /// Get certificates expiring soon.
    async fn get_expiring(&self, days: i32) -> Result<Vec<Certificate>>;
}
