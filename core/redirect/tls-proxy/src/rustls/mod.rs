use std::io::{Error as IoError, ErrorKind, Result as IoResult};
use tokio_rustls::rustls::sign::{self, CertifiedKey};

impl Keycert {
    fn build_certified_key(&mut self) -> IoResult<CertifiedKey> {
        Err(IoError(ErrorKind::Other))
    }
}

