use anyhow::Result;
use dyn_clone::{clone_trait_object, DynClone};

use crate::model::Keycert;

#[async_trait::async_trait()]
pub trait BaseCryptoManager: DynClone {
    async fn get_default_certificate(&self) -> Result<Option<Keycert>>;

    async fn get_certificate(&self, server_name: &str) -> Result<Option<Keycert>>;
}
clone_trait_object!(BaseCryptoManager);
