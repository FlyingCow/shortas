use anyhow::Result;
use dyn_clone::{clone_trait_object, DynClone};

use crate::model::Keycert;

#[async_trait::async_trait()]
pub trait BaseCryptoStore: DynClone {
    async fn store_certificate(&self, route: &Keycert) -> Result<()>;
    async fn update_certificate(&self, route: &Keycert) -> Result<()>;
    async fn delete_certificate(&self, route: &Keycert) -> Result<()>;
    async fn get_certificate(&self, server_name: &str) -> Result<Option<Keycert>>;

    async fn invalidate_certificate(&self, server_name: &str) -> Result<()>;
}
clone_trait_object!(BaseCryptoStore);
