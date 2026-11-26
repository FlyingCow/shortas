use anyhow::Result;

use crate::model::Conversion;

#[async_trait::async_trait()]
pub trait ConversionStore: Send + Sync {
    async fn get_conversion(&self, public_key: &str) -> Result<Option<Conversion>>;
}
