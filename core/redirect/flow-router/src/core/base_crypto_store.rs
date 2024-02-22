
use crate::domain::Keycert;


pub type Result<T> = std::result::Result<T, anyhow::Error>;

pub trait BaseCryptoStore: Send + Sync + Clone {
    fn get_certificate(
        &self,
        server_name: &str,
    ) -> impl std::future::Future<Output = Result<Option<Keycert>>> + Send;
}