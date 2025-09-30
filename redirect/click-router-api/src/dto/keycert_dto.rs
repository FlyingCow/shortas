use serde::{Deserialize, Serialize};
use salvo::oapi::ToSchema;

/// Keycert DTO with PEM encoded string properties
/// 
/// This DTO provides a more API-friendly representation of SSL certificates
/// using PEM encoded strings instead of raw byte vectors.
#[derive(Clone, Serialize, Deserialize, Debug, ToSchema)]
pub struct KeycertDto {
    /// Private key in PEM format
    /// 
    /// The private key encoded as a PEM string, typically starting with
    /// "-----BEGIN PRIVATE KEY-----" or "-----BEGIN RSA PRIVATE KEY-----"
    pub key: String,
    
    /// Certificate in PEM format
    /// 
    /// The certificate encoded as a PEM string, typically starting with
    /// "-----BEGIN CERTIFICATE-----"
    pub cert: String,
    
    /// OCSP response in PEM format
    /// 
    /// The OCSP (Online Certificate Status Protocol) response encoded as a PEM string
    pub ocsp_resp: String,
}

impl Default for KeycertDto {
    fn default() -> Self {
        Self::new()
    }
}

impl KeycertDto {
    /// Create a new KeycertDto with empty PEM strings
    #[inline]
    pub fn new() -> Self {
        Self {
            key: String::new(),
            cert: String::new(),
            ocsp_resp: String::new(),
        }
    }

    /// Create a new KeycertDto with the provided PEM strings
    #[inline]
    pub fn with_pem_data(key: impl Into<String>, cert: impl Into<String>, ocsp_resp: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            cert: cert.into(),
            ocsp_resp: ocsp_resp.into(),
        }
    }

    /// Set the private key PEM string
    #[inline]
    pub fn key(mut self, key: impl Into<String>) -> Self {
        self.key = key.into();
        self
    }

    /// Set the certificate PEM string
    #[inline]
    pub fn cert(mut self, cert: impl Into<String>) -> Self {
        self.cert = cert.into();
        self
    }

    /// Set the OCSP response PEM string
    #[inline]
    pub fn ocsp_resp(mut self, ocsp_resp: impl Into<String>) -> Self {
        self.ocsp_resp = ocsp_resp.into();
        self
    }

    /// Get the private key PEM string
    #[inline]
    pub fn get_key(&self) -> &str {
        &self.key
    }

    /// Get the certificate PEM string
    #[inline]
    pub fn get_cert(&self) -> &str {
        &self.cert
    }

    /// Get the OCSP response PEM string
    #[inline]
    pub fn get_ocsp_resp(&self) -> &str {
        &self.ocsp_resp
    }

    /// Check if the DTO has valid PEM data
    /// 
    /// Returns true if all fields contain non-empty strings
    #[inline]
    pub fn is_valid(&self) -> bool {
        !self.key.is_empty() && !self.cert.is_empty() && !self.ocsp_resp.is_empty()
    }

    /// Check if the DTO has a valid private key
    /// 
    /// Returns true if the key field contains a valid PEM private key
    #[inline]
    pub fn has_valid_key(&self) -> bool {
        self.key.contains("-----BEGIN") && self.key.contains("PRIVATE KEY") && self.key.contains("-----END")
    }

    /// Check if the DTO has a valid certificate
    /// 
    /// Returns true if the cert field contains a valid PEM certificate
    #[inline]
    pub fn has_valid_cert(&self) -> bool {
        self.cert.contains("-----BEGIN") && self.cert.contains("CERTIFICATE") && self.cert.contains("-----END")
    }

    /// Check if the DTO has a valid OCSP response
    /// 
    /// Returns true if the ocsp_resp field contains a valid PEM OCSP response
    #[inline]
    pub fn has_valid_ocsp(&self) -> bool {
        self.ocsp_resp.contains("-----BEGIN") && self.ocsp_resp.contains("-----END")
    }
}

/// Conversion from the original Keycert to KeycertDto
impl From<crate::model::keycert::Keycert> for KeycertDto {
    fn from(keycert: crate::model::keycert::Keycert) -> Self {
        Self {
            key: String::from_utf8_lossy(&keycert.key).to_string(),
            cert: String::from_utf8_lossy(&keycert.cert).to_string(),
            ocsp_resp: String::from_utf8_lossy(&keycert.ocsp_resp).to_string(),
        }
    }
}

/// Conversion from KeycertDto to the original Keycert
impl From<KeycertDto> for crate::model::keycert::Keycert {
    fn from(dto: KeycertDto) -> Self {
        Self {
            key: dto.key.into_bytes(),
            cert: dto.cert.into_bytes(),
            ocsp_resp: dto.ocsp_resp.into_bytes(),
        }
    }
}

/// Conversion from &Keycert to KeycertDto
impl From<&crate::model::keycert::Keycert> for KeycertDto {
    fn from(keycert: &crate::model::keycert::Keycert) -> Self {
        Self {
            key: String::from_utf8_lossy(&keycert.key).to_string(),
            cert: String::from_utf8_lossy(&keycert.cert).to_string(),
            ocsp_resp: String::from_utf8_lossy(&keycert.ocsp_resp).to_string(),
        }
    }
}
