use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
#[non_exhaustive]
pub struct Keycert {
    /// Private key.
    pub key: Vec<u8>,
    /// Certificate.
    pub cert: Vec<u8>,
    /// OCSP response.
    pub ocsp_resp: Vec<u8>,
}

impl Keycert {
    /// Create a new keycert.
    #[inline]
    pub fn new() -> Self {
        Self {
            key: vec![],
            cert: vec![],
            ocsp_resp: vec![],
        }
    }

    /// Sets the Tls private key via bytes slice.
    #[inline]
    pub fn key(mut self, key: impl Into<Vec<u8>>) -> Self {
        self.key = key.into();
        self
    }

    /// Sets the Tls certificate via bytes slice
    #[inline]
    pub fn cert(mut self, cert: impl Into<Vec<u8>>) -> Self {
        self.cert = cert.into();
        self
    }

    /// Get ocsp_resp.
    #[inline]
    pub fn ocsp_resp(&self) -> &[u8] {
        &self.ocsp_resp
    }
}