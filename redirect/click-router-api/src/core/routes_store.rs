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

    /// Atomically store a route family (master + children for conditional routes).
    /// This deletes all existing routes with the same link and inserts the new family.
    /// The operation is atomic - either all routes are stored or none.
    async fn store_route_family(&self, routes: &[Route]) -> Result<()>;

    /// Delete all routes for a domain that match a link prefix.
    /// Used for cleaning up ACME challenge routes after certificate issuance.
    async fn delete_routes_by_switch_and_link_prefix(
        &self,
        switch: &str,
        link_prefix: &str,
    ) -> Result<u64>;
}

clone_trait_object!(RoutesStore);
