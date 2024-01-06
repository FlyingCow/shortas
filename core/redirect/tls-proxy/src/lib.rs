use async_trait::async_trait;

use std::error::Error as StdError;
use std::fs::File;
use std::io::{Error as IoError, Read, Result as IoResult};
use std::path::Path;
use std::sync::Arc;

pub use tracing::{debug, error, info, warn};

pub mod constants;
pub mod dynamodb;
pub mod log;
pub mod moka;
pub mod server;

pub struct Server<'a> {
    crypto_manager: &'a CryptoManager<'a>,
}

impl<'a> Server<'a> {
    pub fn new(crypto_manager: &'a CryptoManager) -> Self {
        Self { crypto_manager }
    }
}

/// Private key and certificate
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct Keycert {
    /// Private key.
    pub key: Vec<u8>,
    /// Certificate.
    pub cert: Vec<u8>,
    /// OCSP response.
    pub ocsp_resp: Vec<u8>,
}

impl Default for Keycert {
    fn default() -> Self {
        Self::new()
    }
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
    /// Sets the Tls private key via File Path, returns [`IoError`] if the file cannot be open.
    #[inline]
    pub fn key_from_path(mut self, path: impl AsRef<Path>) -> IoResult<Self> {
        let mut file = File::open(path.as_ref())?;
        file.read_to_end(&mut self.key)?;
        Ok(self)
    }

    /// Sets the Tls private key via bytes slice.
    #[inline]
    pub fn key(mut self, key: impl Into<Vec<u8>>) -> Self {
        self.key = key.into();
        self
    }

    /// Specify the file path for the TLS certificate to use.
    #[inline]
    pub fn cert_from_path(mut self, path: impl AsRef<Path>) -> IoResult<Self> {
        let mut file = File::open(path)?;
        file.read_to_end(&mut self.cert)?;
        Ok(self)
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

pub struct CryptoManager<'a> {
    crypto_store: &'a dyn CryptoStore,
    crypto_cache: &'a dyn CryptoCache,
}

impl<'a> CryptoManager<'a> {
    pub fn new(crypto_store: &'a dyn CryptoStore, crypto_cache: &'a dyn CryptoCache) -> Self {
        Self {
            crypto_store,
            crypto_cache,
        }
    }

    pub async fn get_certificate(&self, server_name: &str) -> IoResult<Arc<Keycert>> {
        let keycert = self.crypto_cache.get_certificate(server_name.clone()).await;

        if let Ok(keycert) = keycert {
            return Ok(keycert);
        }

        let keycert = self
            .crypto_store
            .get_certificate(server_name.clone())
            .await?;
        //self.crypto_cache.set_certificate(server_name, keycert.clone()).await?;
        Ok(keycert)
    }
}

#[async_trait]
pub trait CryptoStore {
    async fn get_certificate(&self, server_name: &str) -> IoResult<Arc<Keycert>>;
}

#[async_trait]
pub trait CryptoCache {
    async fn get_certificate(&self, server_name: &str) -> IoResult<Arc<Keycert>>;
}

pub type BoxedError = Box<dyn StdError + Send + Sync>;
pub enum Error {
    Io(IoError),
    Other(BoxedError),
}
