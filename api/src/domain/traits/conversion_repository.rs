//! Conversion repository trait.

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::domain::entities::{Conversion, Result};

/// Conversion repository trait for database operations.
#[cfg_attr(any(test, feature = "mocks"), mockall::automock)]
#[async_trait]
pub trait ConversionRepository: Send + Sync {
    async fn create(&self, conversion: &Conversion) -> Result<()>;

    /// Get a conversion, only if it belongs to the workspace.
    async fn get(&self, workspace_id: Uuid, id: Uuid) -> Result<Option<Conversion>>;

    async fn list(&self, workspace_id: Uuid) -> Result<Vec<Conversion>>;

    /// Saves the conversion and returns it with its new version. Fails with a
    /// conflict if it changed since it was read.
    async fn save(&self, conversion: &Conversion) -> Result<Conversion>;

    /// Make conversions active once the router has their routes' coverage.
    async fn finish_activations(&self) -> Result<u64>;

    /// Conversions using the domain that are activating or active.
    async fn list_active_by_domain(&self, domain_id: Uuid) -> Result<Vec<Conversion>>;

    /// How many of the routes belong to the workspace.
    async fn count_routes(&self, workspace_id: Uuid, route_ids: &[Uuid]) -> Result<i64>;
}

/// Looks up the certificate served for a domain.
#[cfg_attr(any(test, feature = "mocks"), mockall::automock)]
#[async_trait]
pub trait CertificateLookup: Send + Sync {
    /// When the domain's current certificate expires, if it has one.
    async fn certificate_expiry(&self, domain: &str) -> anyhow::Result<Option<DateTime<Utc>>>;
}
