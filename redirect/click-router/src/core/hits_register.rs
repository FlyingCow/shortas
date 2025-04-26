use crate::model::Hit;
use anyhow::Result;

#[async_trait::async_trait()]
pub trait HitRegistrar {
    async fn register(&self, hit: Hit) -> Result<()>;
}
