use anyhow::Result;
use dyn_clone::{clone_trait_object, DynClone};

use crate::model::Route;

#[async_trait::async_trait(?Send)]
pub trait BaseRoutesManager: DynClone {
    async fn get_route(
        &self,
        switch: &str,
        domain: &str,
        path: &str,
    ) -> Result<Option<Route>>;
}
clone_trait_object!(BaseRoutesManager);
