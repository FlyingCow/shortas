use anyhow::Result;
use dyn_clone::{clone_trait_object, DynClone};

use crate::model::Keycert;

#[async_trait::async_trait()]
pub trait CryptoStore: DynClone {
    async fn store_certificate(&self, hostname: &str, keycert: &Keycert) -> Result<()>;
    async fn update_certificate(&self, hostname: &str, keycert: &Keycert) -> Result<()>;
    async fn delete_certificate(&self, hostname: &str) -> Result<()>;
    async fn get_certificate(&self, hostname: &str) -> Result<Option<Keycert>>;

    async fn invalidate_certificate(&self, hostname: &str) -> Result<()>;
}
clone_trait_object!(CryptoStore);
