use anyhow::Result;
use flume::Sender;
use tokio_util::sync::CancellationToken;

use super::Hit;

#[async_trait::async_trait]
pub trait HitStreamSource {
    async fn pull(&self, ts: Sender<Hit>, token: CancellationToken) -> Result<()>;
}
