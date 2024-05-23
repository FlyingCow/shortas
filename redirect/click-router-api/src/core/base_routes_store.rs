use anyhow::Result;
use dyn_clone::{clone_trait_object, DynClone};

use crate::model::Route;

#[async_trait::async_trait(?Send)]
pub trait BaseRoutesStore: DynClone {
    async fn store_route(&self, route: &Route) -> Result<()>;
    async fn update_route(&self, route: &Route) -> Result<()>;
    async fn delete_route(&self, route: &Route) -> Result<()>;
    async fn get_route(&self, switch: &str, domain: &str, path: &str) -> Result<Option<Route>>;
    async fn invalidate_route(&self, switch: &str, domain: &str, path: &str) -> Result<()>;
}

clone_trait_object!(BaseRoutesStore);