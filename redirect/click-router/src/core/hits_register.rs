use anyhow::Result;
use dyn_clone::{clone_trait_object, DynClone};

use crate::model::Hit;

#[async_trait::async_trait()]
pub trait BaseHitRegistrar: DynClone {
    async fn register(&self, hit: Hit) -> Result<()>;
}
clone_trait_object!(BaseHitRegistrar);
