use anyhow::Result;
use dyn_clone::{clone_trait_object, DynClone};

use crate::model::Keycert;

#[async_trait::async_trait(?Send)]
pub trait BaseCryptoStore: DynClone {
    async fn get_certificate(&self, server_name: &str) -> Result<Option<Keycert>>;

    async fn invalidate(&self, server_name: &str) -> Result<()>;
}
clone_trait_object!(BaseCryptoStore);
