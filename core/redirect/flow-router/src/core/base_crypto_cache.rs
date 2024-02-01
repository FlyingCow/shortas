use crate::domain::Keycert;

pub type Result<T> = std::result::Result<T, anyhow::Error>;

pub trait BaseCryptoCache: Send + Sync + Clone {
    fn get_certificate(
        &self,
        server_name: &str,
        init: impl std::future::Future<Output = Option<Keycert>> + Send
    ) -> impl std::future::Future<Output = Result<Option<Keycert>>> + Send;

    fn remove_certificate(&self,
        server_name: &str
    ) -> impl std::future::Future<Output = Result<()>> + Send;
}