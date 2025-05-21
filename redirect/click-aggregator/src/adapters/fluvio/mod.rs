use anyhow::Result;
use fluvio::{
    consumer::{ConsumerConfigExtBuilder, ConsumerStream, OffsetManagementStrategy},
    Fluvio, FluvioClusterConfig, Offset,
};
use settings::ClickStreamConfig;
use std::{sync::mpsc::SyncSender, time::Duration};
use tokio::task::JoinHandle;

use tokio_util::sync::CancellationToken;

use crate::core::{ClickStreamItem, ClickStreamSource};

pub mod settings;

pub struct FluvioHitStream {
    settings: ClickStreamConfig,
}

impl FluvioHitStream {
    pub fn new(settings: ClickStreamConfig) -> Self {
        Self { settings }
    }
}

#[async_trait::async_trait]
impl ClickStreamSource for FluvioHitStream {
    async fn pull(
        &self,
        ts: SyncSender<ClickStreamItem>,
        token: CancellationToken,
    ) -> Result<JoinHandle<()>> {
        let settings = self.settings.clone();

        let handler = tokio::spawn(async move {
            use futures_lite::StreamExt;

            let fluvio = Fluvio::connect_with_config(&FluvioClusterConfig::new(settings.host))
                .await
                .expect("Can not connect to fluvio cluster.");

            let mut stream = fluvio
                .consumer_with_config(
                    ConsumerConfigExtBuilder::default()
                        .topic(settings.topic)
                        .offset_consumer(settings.consumer)
                        .offset_start(Offset::beginning())
                        .offset_strategy(OffsetManagementStrategy::Auto)
                        .offset_flush(Duration::from_millis(1000))
                        .build()
                        .expect("Can not create fluvio hits consumer config."),
                )
                .await
                .expect("Can not create fluvio hits consumer.");

            while let Some(Ok(record)) = stream.next().await {
                let hit = serde_json::from_slice(record.as_ref())
                    .expect("Can not deserialize hit object.");

                ts.send(hit).expect("Can not re-send a hit to consumer.");

                if token.is_cancelled() {
                    break;
                }
            }

            // synchronously flush for shutdown (or none if intentionally ending processing)
            let _ = stream.offset_flush().await;
        });

        Ok(handler)
    }
}
