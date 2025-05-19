pub mod fluvio;
pub mod kafka;

use anyhow::Result;
use std::sync::mpsc::SyncSender;
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;

use crate::{
    core::{ClickStreamItem, ClickStreamSource},
    FluvioHitStream, KafkaHitStream,
};

pub enum ClickStreamSourceType {
    Kafka(KafkaHitStream),
    Fluvio(FluvioHitStream),
}

#[async_trait::async_trait]
impl ClickStreamSource for ClickStreamSourceType {
    async fn pull(
        &self,
        ts: SyncSender<ClickStreamItem>,
        token: CancellationToken,
    ) -> Result<JoinHandle<()>> {
        match self {
            ClickStreamSourceType::Kafka(stream) => stream.pull(ts, token).await,
            ClickStreamSourceType::Fluvio(stream) => stream.pull(ts, token).await,
        }
    }
}
