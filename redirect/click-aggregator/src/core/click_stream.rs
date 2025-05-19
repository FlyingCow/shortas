use std::sync::mpsc::SyncSender;
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;

use anyhow::Result;

use super::ClickStreamItem;

#[async_trait::async_trait]
pub trait ClickStreamSource {
    async fn pull(
        &self,
        ts: SyncSender<ClickStreamItem>,
        token: CancellationToken,
    ) -> Result<JoinHandle<()>>;
}
