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

/// When a PEM certificate expires.
pub fn certificate_expiry(cert_pem: &[u8]) -> Option<DateTime<Utc>> {
    use x509_parser::prelude::*;

    let (_, pem) = x509_parser::pem::parse_x509_pem(cert_pem).ok()?;
    let (_, cert) = X509Certificate::from_der(&pem.contents).ok()?;
    DateTime::from_timestamp(cert.validity().not_after.timestamp(), 0)
}
