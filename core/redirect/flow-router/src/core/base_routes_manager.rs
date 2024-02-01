use std::fmt;
use thiserror::Error;

use crate::domain::Route;

pub type Result<T> = std::result::Result<T, RoutesManagerError>;

pub trait BaseRoutesManager: Send + Sync + Clone {

    fn get_route(
        &self,
        switch: &str,
        domain: &str,
        path: &str,
    ) -> impl std::future::Future<Output = Result<Option<Route>>> + Send;
}

#[derive(Error, Debug)]
pub enum RoutesManagerError {
    #[error("unknown data store error")]
    Other(RoutesManagerOtherError),
}

#[derive(Error, Debug)]
pub struct RoutesManagerOtherError {
    msg: String,
    #[source]  // optional if field name is `source`
    source: anyhow::Error,
}

impl fmt::Display for RoutesManagerOtherError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}