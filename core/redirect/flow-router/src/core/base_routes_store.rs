use crate::domain::Route;


pub type Result<T> = std::result::Result<T, anyhow::Error>;

pub trait BaseRoutesStore: Send + Sync + Clone {
    fn get_route(
        &self,
        switch: &str,
        path: &str,
    ) -> impl std::future::Future<Output = Result<Option<Route>>> + Send;
}