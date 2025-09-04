use anyhow::Result;
use flume::Sender;
use fluvio::{
    consumer::{ConsumerConfigExtBuilder, ConsumerStream, OffsetManagementStrategy},
    Fluvio, FluvioClusterConfig, Offset,
};
use futures::StreamExt;
use settings::ClickStreamConfig;
use std::time::Duration;

use tokio_util::sync::CancellationToken;

use crate::core::{ClickStreamItem, ClickStreamSource};

pub mod settings;

pub struct FluvioHitStream {
    settings: ClickStreamConfig,
    fluvio: Fluvio,
}
impl FluvioHitStream {
    pub async fn connect(settings: ClickStreamConfig) -> Self {
        let fluvio = Fluvio::connect_with_config(&FluvioClusterConfig::new(settings.host.clone()))
            .await
            .expect("Can not connect to fluvio cluster.");
        Self { settings, fluvio }
    }
}

#[async_trait::async_trait]
impl ClickStreamSource for FluvioHitStream {
    async fn pull(&self, ts: Sender<ClickStreamItem>, token: CancellationToken) -> Result<()> {
        let settings = self.settings.clone();

        let config = ConsumerConfigExtBuilder::default()
            .topic(settings.topic)
            .offset_consumer(settings.consumer)
            .offset_start(Offset::beginning())
            .offset_strategy(OffsetManagementStrategy::Auto)
            .offset_flush(Duration::from_millis(10000))
            .max_bytes(100000000)
            .build()
            .expect("Failed to build consumer config");

        let mut stream = self
            .fluvio
            .consumer_with_config(config)
            .await
            .expect("Can not create fluvio hits consumer.");

        while let Some(Ok(record)) = stream.next().await {
            let hit =
                serde_json::from_slice(record.as_ref()).expect("Can not deserialize hit object.");

            ts.send(hit).expect("Can not re-send a hit to consumer.");

            if token.is_cancelled() {
                break;
            }
        }

        // synchronously flush for shutdown (or none if intentionally ending processing)
        let _ = stream.offset_flush().await;

        Ok(())
    }
}
