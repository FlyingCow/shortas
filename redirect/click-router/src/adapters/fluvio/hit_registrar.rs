use std::time::Duration;

use anyhow::Result;
use tokio::sync::mpsc;

use crate::{core::hits_register::HitRegistrar, model::Hit};

use super::{producer::FluvioProducer, settings::HitStreamConfig};

/// Clicks kept while the stream is unavailable.
const QUEUE_SIZE: usize = 20_000;
const SEND_TIMEOUT: Duration = Duration::from_secs(10);
const RETRY_DELAY: Duration = Duration::from_secs(1);

/// Sends clicks in the background, so redirects never wait for the stream.
#[derive(Clone)]
pub struct FluvioHitRegistrar {
    queue: mpsc::Sender<Vec<u8>>,
}

impl FluvioHitRegistrar {
    pub fn new(settings: &HitStreamConfig) -> Self {
        let producer = FluvioProducer::new(settings);
        let (queue, mut records) = mpsc::channel::<Vec<u8>>(QUEUE_SIZE);

        tokio::spawn(async move {
            while let Some(record) = records.recv().await {
                while let Err(e) = producer.send(record.clone(), false, SEND_TIMEOUT).await {
                    tracing::error!(error = %e, queued = records.len(), "Failed to send a click");
                    tokio::time::sleep(RETRY_DELAY).await;
                }
            }
        });

        Self { queue }
    }
}

#[async_trait::async_trait()]
impl HitRegistrar for FluvioHitRegistrar {
    async fn register(&self, hit: &Hit) -> Result<()> {
        self.queue
            .try_send(serde_json::to_vec(hit)?)
            .map_err(|_| anyhow::anyhow!("the click queue is full"))
    }
}
