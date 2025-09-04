use flume::Sender;
use tokio_util::sync::CancellationToken;

use anyhow::Result;

use super::ClickStreamItem;

#[async_trait::async_trait]
pub trait ClickStreamSource {
    async fn pull(&self, ts: Sender<ClickStreamItem>, token: CancellationToken) -> Result<()>;
}

#[async_trait::async_trait]
pub trait ClickStreamStore {
    async fn register(&self, click: &ClickStreamItem) -> Result<()>;
}
