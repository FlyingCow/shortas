use anyhow::Result;
use dyn_clone::{clone_trait_object, DynClone};

use crate::model::Hit;

#[async_trait::async_trait]
pub trait BaseHitStream: DynClone {
    async fn pull(&mut self) -> Result<Vec<Hit>>;
}

clone_trait_object!(BaseHitStream);
