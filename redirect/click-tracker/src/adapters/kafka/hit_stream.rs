use std::{
    sync::{mpsc::Sender, Arc},
    time::Duration,
};

use super::settings::HitStreamConfig;
use crate::{core::hit_stream::BaseHitStream, model::Hit};
use anyhow::{Ok, Result};
use kafka::{
    client::{FetchOffset, GroupOffsetStorage},
    consumer::Consumer,
};
use tokio::{sync::Mutex, time::sleep};
use tokio_util::sync::CancellationToken;

const TRACKERS_GROUP: &str = "trackers";
const IDLE_TIMEOUT: u64 = 500;
const MAX_BYTES_FETCH: i32 = 55728640;

#[derive(Clone)]
pub struct KafkaHitStream {
    consumer: Arc<Mutex<Consumer>>,
}

impl KafkaHitStream {
    pub fn new(settings: HitStreamConfig) -> Self {
        let consumer = Consumer::from_hosts(settings.hosts.clone())
            .with_topic(settings.topic.clone())
            .with_fallback_offset(FetchOffset::Earliest)
            .with_group(TRACKERS_GROUP.to_string())
            .with_offset_storage(Some(GroupOffsetStorage::Kafka))
            .with_fetch_max_bytes_per_partition(MAX_BYTES_FETCH)
            .create()
            .unwrap();

        Self {
            consumer: Arc::new(Mutex::new(consumer)),
        }
    }
}

#[async_trait::async_trait()]
impl BaseHitStream for KafkaHitStream {
    async fn pull(&mut self, tx: Sender<Hit>, cancelation_token: CancellationToken) -> Result<()> {
        let mut consumer = self.consumer.lock().await;

        // while let Some(Ok(record)) = stream.next().await {
        //     let string = String::from_utf8_lossy(record.value());
        //     let hit = serde_json::from_slice(string.as_bytes())?;

        //     tx.send(hit);

        //     if cancelation_token.is_cancelled() {
        //         break;
        //     }
        // }

        // let mut hits: Vec<Hit> = vec![];
        // for ms in consumer.poll()?.iter() {
        //     for m in ms.messages() {
        //         let hit = serde_json::from_slice(m.value)?;
        //         hits.push(hit);
        //     }
        //     let _ = consumer.consume_messageset(ms);
        // }
        // consumer.commit_consumed().unwrap();

        // if hits.len() == 0 {
        //     sleep(Duration::from_millis(IDLE_TIMEOUT)).await;
        // }

        // if cancelation_token.is_cancelled() {
        //     break;
        // }

        Ok(())
    }
}
