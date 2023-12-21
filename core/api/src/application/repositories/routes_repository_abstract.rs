use async_trait::async_trait;
use std::error::Error;

use crate::domain::route_entity::RouteEntity;

#[cfg(test)]
use mockall::{predicate::*, *};

#[cfg_attr(test, automock)]
#[async_trait(?Send)]
pub trait RoutesRepositoryAbstract {
    async fn get_route(
        &self,
        switch: String,
        link: String,
    ) -> Result<Option<RouteEntity>, Box<dyn Error>>;
    async fn create_route(&self, route: RouteEntity) -> Result<(), Box<dyn Error>>;
    async fn update_route(&self, route: RouteEntity) -> Result<(), Box<dyn Error>>;
    async fn delete_route(&self, switch: String, link: String) -> Result<(), Box<dyn Error>>;
}

