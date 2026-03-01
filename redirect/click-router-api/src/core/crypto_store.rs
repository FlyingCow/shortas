use anyhow::Result;
use chrono::{DateTime, Utc};
use dyn_clone::{clone_trait_object, DynClone};

use crate::model::{CertificateInfo, Keycert};

#[async_trait::async_trait()]
pub trait CryptoStore: DynClone {
    async fn store_certificate(&self, hostname: &str, keycert: &Keycert) -> Result<()>;
    async fn store_certificate_with_owner(
        &self,
        hostname: &str,
        keycert: &Keycert,
        owner_id: Option<&str>,
    ) -> Result<()>;
    async fn update_certificate(&self, hostname: &str, keycert: &Keycert) -> Result<()>;
    async fn delete_certificate(&self, hostname: &str) -> Result<()>;
    async fn get_certificate(&self, hostname: &str) -> Result<Option<Keycert>>;
    async fn get_certificates_expiring_before(
        &self,
        before: DateTime<Utc>,
    ) -> Result<Vec<CertificateInfo>>;

    async fn invalidate_certificate(&self, hostname: &str) -> Result<()>;
}
clone_trait_object!(CryptoStore);
