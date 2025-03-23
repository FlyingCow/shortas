use std::{
    sync::{mpsc::Sender, Arc},
    time::Duration,
};

use super::settings::HitStreamConfig;
use crate::{core::hit_stream::BaseHitStream, model::Hit};
use anyhow::Result;
use fluvio::{
    consumer::{ConsumerConfigExtBuilder, OffsetManagementStrategy},
    Fluvio, FluvioClusterConfig, Offset,
};
use futures_util::StreamExt;
use tokio::time::sleep;
use tokio_util::sync::CancellationToken;

const TRACKERS_GROUP: &str = "trackers";
const IDLE_TIMEOUT: u64 = 500;
const MAX_BYTES_FETCH: i32 = 55728640;

#[derive(Clone)]
pub struct FluvioHitStream {
    consumer: Arc<Fluvio>,
    settings: HitStreamConfig,
}

impl FluvioHitStream {
    pub async fn new(settings: HitStreamConfig) -> Self {
        // Connect to Fluvio cluster

        let config = FluvioClusterConfig::new(settings.host.clone());

        // Connet to fluvio cluster & create a producer
        let fluvio = Fluvio::connect_with_config(&config)
            .await
            .expect("Failed to connect to Fluvio");

        Self {
            consumer: Arc::new(fluvio),
            settings,
        }
    }
}

#[async_trait::async_trait()]
impl BaseHitStream for FluvioHitStream {
    async fn pull(&mut self, tx: Sender<Hit>, cancelation_token: CancellationToken) -> Result<()> {
        // Consume last record from topic
        let config = ConsumerConfigExtBuilder::default()
            .topic(self.settings.topic.clone())
            //.partition(0)
            .offset_start(Offset::end())
            .offset_consumer(TRACKERS_GROUP.to_string())
            .offset_strategy(OffsetManagementStrategy::Auto)
            .build()
            .expect("Failed to build consumer config");

        // Create consumer & stream one record
        let mut stream = self
            .consumer
            .consumer_with_config(config)
            .await
            .expect("Failed to create consumer")
            .boxed();

        tokio::spawn(async move {
            while let Some(Ok(record)) = stream.next().await {
                let string = String::from_utf8_lossy(record.value());
                let hit =
                    serde_json::from_slice(string.as_bytes()).expect("Can not deserialize a hit");

                tx.send(hit).expect("Can not send a hit to the pipe");

                if cancelation_token.is_cancelled() {
                    break;
                }
            }
        });

        Ok(())
    }
}
