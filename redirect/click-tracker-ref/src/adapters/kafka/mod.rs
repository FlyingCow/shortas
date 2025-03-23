use anyhow::Result;
use kafka::{
    client::RequiredAcks,
    producer::{Producer, Record},
};
use settings::ClickAggsConfig;
use std::{
    sync::{Arc, mpsc::SyncSender},
    time::Duration,
};
use tokio::{sync::Mutex, task::JoinHandle};

use tokio_util::sync::CancellationToken;

use crate::core::{ClickStreamItem, Hit, HitStreamSource, aggs::ClickAggsRegistrar};

pub mod settings;

#[allow(dead_code)]
pub struct KafkaHitStream;

#[async_trait::async_trait]
impl HitStreamSource for KafkaHitStream {
    async fn pull(&self, ts: SyncSender<Hit>, token: CancellationToken) -> Result<JoinHandle<()>> {
        let handler = tokio::spawn(async move {
            let mut iteration = 0u64;
            let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(1));
            loop {
                interval.tick().await;
                if token.is_cancelled() {
                    break;
                }
                iteration = iteration + 1;

                println!("sending {}-{}", "kafka", iteration);
                //ts.send().unwrap();
            }
        });

        Ok(handler)
    }
}

#[derive(Clone)]
pub struct KafkaClickAggsRegistrar {
    settings: ClickAggsConfig,
    producer: Arc<Mutex<Producer>>,
}

impl KafkaClickAggsRegistrar {
    pub fn new(settings: ClickAggsConfig) -> Self {
        let producer = Producer::from_hosts(settings.hosts.clone())
            .with_ack_timeout(Duration::from_secs(settings.ack_timeout_secs))
            .with_required_acks(RequiredAcks::One)
            .create()
            .unwrap();

        Self {
            producer: Arc::new(Mutex::new(producer)),
            settings,
        }
    }
}

#[async_trait::async_trait()]
impl ClickAggsRegistrar for KafkaClickAggsRegistrar {
    async fn register(&self, click: ClickStreamItem) -> Result<()> {
        let record = Record::from_value(
            self.settings.topic.as_str(),
            serde_json::to_vec(&click).unwrap(),
        );

        self.producer.lock().await.send(&record)?;

        Ok(())
    }
}
