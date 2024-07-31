use anyhow::Result;
use dyn_clone::{clone_trait_object, DynClone};

use crate::model::ClickStreamItem;

#[async_trait::async_trait()]
pub trait BaseClickAggsRegistrar: DynClone {
    async fn register(&self, click: ClickStreamItem) -> Result<()>;
}
clone_trait_object!(BaseClickAggsRegistrar);
