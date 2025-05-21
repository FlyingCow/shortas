pub mod clickhouse;
pub mod fluvio;
pub mod kafka;

use anyhow::Result;
use clickhouse::ClickhouseClickStreamStore;
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

#[async_trait::async_trait]
pub trait ClickStreamStore {
    async fn register(&mut self, click: &ClickStreamItem) -> Result<()>;
}

#[derive(Clone)]
pub enum ClickStreamStoreType {
    Clickhouse(ClickhouseClickStreamStore),
}

#[async_trait::async_trait]
impl ClickStreamStore for ClickStreamStoreType {
    async fn register(&mut self, click: &ClickStreamItem) -> Result<()> {
        match self {
            ClickStreamStoreType::Clickhouse(store) => store.register(&click).await,
        }
    }
}
