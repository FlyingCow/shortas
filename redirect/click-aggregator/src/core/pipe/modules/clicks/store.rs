use anyhow::Result;
use tracing::info;

use crate::{
    adapters::{ClickStreamStore, ClickStreamStoreType},
    core::{aggs_pipe::AggsModule, AggsPipeContext},
};

#[derive(Clone)]
pub struct StoreModule {
    click_stream_store: ClickStreamStoreType,
}

#[async_trait::async_trait()]
impl AggsModule for StoreModule {
    async fn execute(&mut self, context: &mut AggsPipeContext) -> Result<()> {
        info!("{}", serde_json::json!(context.click));
        self.click_stream_store.register(&context.click).await
    }
}

impl StoreModule {
    pub fn new(click_stream_store: ClickStreamStoreType) -> Self {
        Self { click_stream_store }
    }
}
