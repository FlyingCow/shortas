use std::time::Duration;

use anyhow::Result;

use aws_config::SdkConfig;
use aws_sdk_kinesis::Client;
use aws_sdk_kinesis::{primitives::Blob, types::builders::PutRecordsRequestEntryBuilder};
use rand::seq::SliceRandom;
use tracing::warn;

use crate::{
    core::hits_register::BaseHitRegistrar,
    model::Hit,
    utils::async_queue::{AsyncQueue, BatchProcess},
};

use super::settings::HitStreamConfig;

struct KinesisHitsProducer {
    client: Client,
    settings: HitStreamConfig,
}

impl Into<Vec<u8>> for &Hit {
    fn into(self) -> Vec<u8> {
        serde_json::to_vec(self).unwrap()
    }
}

#[async_trait::async_trait()]
impl BatchProcess<Hit> for KinesisHitsProducer {
    async fn process(&mut self, batch: Vec<Hit>) -> Result<()> {
        //let blob = Blob::new(*batch);
        warn!("batch");
        let key = self
            .settings
            .partition_keys
            .choose(&mut rand::thread_rng())
            .unwrap();

        let entries = batch
            .iter()
            .map(|hit| {
                let blob = Blob::new(hit);

                PutRecordsRequestEntryBuilder::default()
                    .partition_key(key)
                    .data(blob)
                    .build()
                    .unwrap()
            })
            .collect::<Vec<_>>();

        self.client
            .put_records()
            .set_records(Some(entries))
            .stream_name(&self.settings.stream_name)
            .send()
            .await?;

        Ok(())
    }
}

impl KinesisHitsProducer {
    pub fn new(sdk_config: &SdkConfig, settings: HitStreamConfig) -> Self {
        Self {
            settings,
            client: Client::new(sdk_config),
        }
    }
}

#[derive(Clone, Debug)]
pub struct KinesisHitRegistrar {
    queue: AsyncQueue<Hit>,
}

impl KinesisHitRegistrar {
    pub fn new(sdk_config: &SdkConfig, settings: HitStreamConfig) -> Self {
        let iteration_duration = settings.iteration_seconds;
        let consumers_count = settings.consumers_count;
        let batch_size = settings.batch_size;

        let hit_processor = KinesisHitsProducer::new(sdk_config, settings);
        let hits_queue = AsyncQueue::new(
            Box::new(hit_processor),
            batch_size,
            consumers_count,
            Duration::from_secs(iteration_duration),
        );

        Self { queue: hits_queue }
    }
}

#[async_trait::async_trait()]
impl BaseHitRegistrar for KinesisHitRegistrar {
    async fn register(&self, hit: Hit) -> Result<()> {
        self.queue.enqueue(hit).await
    }
}
