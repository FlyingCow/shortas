use anyhow::Result;
use fluvio::{
    Fluvio, FluvioClusterConfig, Offset, TopicProducer, TopicProducerConfigBuilder,
    consumer::{ConsumerConfigExtBuilder, ConsumerStream, OffsetManagementStrategy},
    spu::SpuSocketPool,
};
use settings::{ClickAggsConfig, HitStreamConfig};
use std::{sync::mpsc::SyncSender, time::Duration};
use tokio::task::JoinHandle;

use tokio_util::sync::CancellationToken;

use crate::core::{ClickStreamItem, Hit, HitStreamSource, aggs::ClickAggsRegistrar};

pub mod settings;

#[allow(dead_code)]
pub struct FluvioHitStream {
    settings: HitStreamConfig,
}

impl FluvioHitStream {
    pub fn new(settings: HitStreamConfig) -> Self {
        Self { settings }
    }
}

#[async_trait::async_trait]
impl HitStreamSource for FluvioHitStream {
    async fn pull(&self, ts: SyncSender<Hit>, token: CancellationToken) -> Result<JoinHandle<()>> {
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

#[derive(Clone)]
pub struct FluvioClickAggsRegistrar {
    producer: TopicProducer<SpuSocketPool>,
    token: CancellationToken,
}

impl FluvioClickAggsRegistrar {
    pub async fn new(settings: &ClickAggsConfig, token: CancellationToken) -> Self {
        let fluvio = Fluvio::connect_with_config(&FluvioClusterConfig::new(&settings.host))
            .await
            .expect("Can not connect to fluvio cluster.");

        let producer = fluvio
            .topic_producer_with_config(
                settings.topic.clone(),
                TopicProducerConfigBuilder::default()
                    .linger(Duration::from_millis(settings.linger_millis))
                    .batch_size(settings.batch_size)
                    .build()
                    .expect("Can not build click aggs registrar topic config."),
            )
            .await
            .expect("Can not build click aggs registrar topic producer.");

        Self { producer, token }
    }
}

#[async_trait::async_trait()]
impl ClickAggsRegistrar for FluvioClickAggsRegistrar {
    async fn register(&self, click: ClickStreamItem) -> Result<()> {
        self.producer
            .send(click.id.as_str(), serde_json::to_string(&click)?)
            .await?;

        if self.token.is_cancelled() {
            self.producer.flush().await?
        }

        Ok(())
    }
}
