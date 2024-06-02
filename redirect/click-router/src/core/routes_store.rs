use anyhow::Result;
use dyn_clone::{clone_trait_object, DynClone};

use crate::model::Route;

#[async_trait::async_trait()]
pub trait BaseRoutesStore: DynClone {
    async fn get_route(&self, switch: &str, path: &str) -> Result<Option<Route>>;
    async fn invalidate(&self, switch: &str, path: &str) -> Result<()>;
}
clone_trait_object!(BaseRoutesStore);