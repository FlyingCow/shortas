use anyhow::Result;

use super::ClickStreamItem;

#[async_trait::async_trait()]
pub trait ClickAggsRegistrar {
    async fn register(&self, click: ClickStreamItem) -> Result<()>;
}
