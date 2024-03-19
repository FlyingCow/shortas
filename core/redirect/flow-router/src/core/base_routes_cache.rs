use crate::domain::Route;

pub type Result<T> = std::result::Result<T, anyhow::Error>;

pub trait BaseRoutesCache: Send + Sync + Clone {
    fn get_route(
        &self,
        switch: &str,
        path: &str,
        init: impl std::future::Future<Output = Option<Route>> + Send,
    ) -> impl std::future::Future<Output = Result<Option<Route>>> + Send;

    fn remove_route(
        &self,
        switch: &str,
        path: &str,
    ) -> impl std::future::Future<Output = Result<()>> + Send;
}

//Even null certificates should be cached to minimize db hit
//in case there is no certificate for a specified domain name
#[derive(Clone, Debug)]
pub struct RouteContainer {
    pub value: Option<Route>,

    ///Specifies if current container is coming from cache
    ///If false - db should be hit and the cache fulfilled
    pub from_cache: bool,
}
