use std::sync::mpsc::SyncSender;
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;

use anyhow::Result;

use super::Hit;

#[async_trait::async_trait]
pub trait HitStreamSource {
    async fn pull(&self, ts: SyncSender<Hit>, token: CancellationToken) -> Result<JoinHandle<()>>;
}
