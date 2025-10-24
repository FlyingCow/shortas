use anyhow::{Context, Result};
use flume::Sender;
use fluvio::{
    Compression, Fluvio, FluvioClusterConfig, Offset, RecordKey, TopicProducer,
    TopicProducerConfigBuilder,
    consumer::{ConsumerConfigExtBuilder, ConsumerStream, OffsetManagementStrategy},
    spu::SpuSocketPool,
};
use futures::StreamExt;
use settings::{ClickAggsConfig, HitStreamConfig};
use std::time::Duration;
use tokio::time::timeout;
use tokio_util::sync::CancellationToken;

use crate::core::{ClickStreamItem, Hit, HitStreamSource, aggs::ClickAggsRegistrar};

const FLUVIO_SEND_TIMEOUT_SECS: u64 = 10;  // 10 second timeout for Fluvio send operations

pub mod settings;

#[allow(dead_code)]
pub struct FluvioHitStream {
    settings: HitStreamConfig,
    fluvio: Fluvio,
}

impl FluvioHitStream {
    pub async fn connect(settings: HitStreamConfig) -> Self {
        let fluvio = Fluvio::connect_with_config(&FluvioClusterConfig::new(settings.host.clone()))
            .await
            .expect("Can not connect to fluvio cluster.");
        Self { settings, fluvio }
    }
}

#[async_trait::async_trait]
impl HitStreamSource for FluvioHitStream {
    async fn pull(&self, ts: Sender<Hit>, token: CancellationToken) -> Result<()> {
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

#[derive(Clone)]
pub struct FluvioClickAggsRegistrar {
    producer: TopicProducer<SpuSocketPool>,
    token: CancellationToken,
}

impl FluvioClickAggsRegistrar {
    pub async fn new(settings: &ClickAggsConfig, token: CancellationToken) -> Self {
        // Use config builder to create a topic producer config
        let producer_config = TopicProducerConfigBuilder::default()
            .batch_size(settings.batch_size)
            .linger(Duration::from_millis(settings.linger_millis))
            .compression(Compression::Gzip)
            .build()
            .expect("Failed to create topic producer config");

        let config = FluvioClusterConfig::new(&settings.host);

        // Connet to fluvio cluster & create a producer
        let fluvio = Fluvio::connect_with_config(&config)
            .await
            .expect("Failed to connect to Fluvio");

        let producer = fluvio
            .topic_producer_with_config(&settings.topic, producer_config)
            .await
            .expect("Failed to create a producer");

        Self { producer, token }
    }
}

#[async_trait::async_trait()]
impl ClickAggsRegistrar for FluvioClickAggsRegistrar {
    async fn register(&self, click: ClickStreamItem) -> Result<()> {
        let record = serde_json::to_vec(&click).unwrap();

        // Add timeout to prevent hanging on Fluvio send operations
        let send_operation = self.producer.send(RecordKey::NULL, record);

        timeout(Duration::from_secs(FLUVIO_SEND_TIMEOUT_SECS), send_operation)
            .await
            .context("Fluvio send operation timed out")?
            .context("Failed to send record to Fluvio")?;

        if self.token.is_cancelled() {
            // Also add timeout to flush operation
            let flush_operation = self.producer.flush();
            timeout(Duration::from_secs(FLUVIO_SEND_TIMEOUT_SECS), flush_operation)
                .await
                .context("Fluvio flush operation timed out")??;
        }

        Ok(())
    }
}
