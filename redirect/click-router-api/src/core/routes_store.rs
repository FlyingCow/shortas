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

    /// Get all routes with the same link (route family: master + children)
    async fn get_routes_by_link(&self, link: &str) -> Result<Vec<Route>>;

    /// Delete all routes with the same link (cascade delete for route families)
    async fn delete_routes_by_link(&self, link: &str) -> Result<u64>;
}

clone_trait_object!(RoutesStore);
