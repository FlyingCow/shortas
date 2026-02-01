use anyhow::Result;
use dyn_clone::{clone_trait_object, DynClone};

use crate::model::Route;

#[async_trait::async_trait()]
pub trait RoutesStore: DynClone {
    async fn store_route(&self, route: &Route) -> Result<()>;
    async fn update_route(&self, route: &Route) -> Result<()>;
    async fn delete_route(&self, route: &Route) -> Result<()>;
    async fn get_route(&self, switch: &str, link: &str) -> Result<Option<Route>>;
    async fn get_route_by_route_id(&self, route_id: &str) -> Result<Option<Route>>;
    async fn invalidate_route(&self, switch: &str, link: &str) -> Result<()>;
}

clone_trait_object!(RoutesStore);
