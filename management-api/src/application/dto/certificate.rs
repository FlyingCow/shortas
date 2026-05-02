//! Certificate DTOs for API requests and responses.

use salvo::oapi::ToSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::entities::Certificate;

/// Certificate response DTO.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CertificateDto {
    pub id: String,
    pub domain_id: String,
    pub owner_id: String,
    /// Masked certificate info (not the actual key).
    pub has_key: bool,
    pub has_cert: bool,
    pub has_ocsp: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
}

impl CertificateDto {
    pub fn from_entity(cert: Certificate) -> Self {
        Self {
            id: cert.id.to_string(),
            domain_id: cert.domain_id.to_string(),
            owner_id: cert.owner_id,
            has_key: !cert.key.is_empty(),
            has_cert: !cert.cert.is_empty(),
            has_ocsp: cert.ocsp_resp.is_some(),
            expires_at: cert.expires_at.map(|dt| dt.to_rfc3339()),
            created_at: cert.created_at.map(|dt| dt.to_rfc3339()),
        }
    }
}

/// Certificate creation request DTO.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateCertificateDto {
    pub domain_id: String,
    pub key: String,
    pub cert: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ocsp_resp: Option<String>,
}

impl CreateCertificateDto {
    pub fn to_entity(self, owner_id: &str) -> Result<Certificate, String> {
        let domain_id = Uuid::parse_str(&self.domain_id)
            .map_err(|_| "Invalid domain_id")?;

        let mut cert = Certificate::new(self.key, self.cert, owner_id.to_string(), domain_id);
        cert.ocsp_resp = self.ocsp_resp;
        Ok(cert)
    }
}

/// Certificate update request DTO.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UpdateCertificateDto {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cert: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ocsp_resp: Option<String>,
}

impl UpdateCertificateDto {
    pub fn apply_to(self, mut certificate: Certificate) -> Certificate {
        if let Some(key) = self.key {
            certificate.key = key;
        }
        if let Some(cert) = self.cert {
            certificate.cert = cert;
        }
        if let Some(ocsp) = self.ocsp_resp {
            certificate.ocsp_resp = Some(ocsp);
        }
        certificate
    }
}
