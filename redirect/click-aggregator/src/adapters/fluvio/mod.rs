use anyhow::{Context, Result};
use flume::Sender;
use fluvio::{
    consumer::{ConsumerConfigExtBuilder, ConsumerStream, OffsetManagementStrategy},
    Fluvio, FluvioClusterConfig, Offset,
};
use futures::StreamExt;
use settings::ClickStreamConfig;
use std::time::Duration;
use tracing::{info, warn};

use tokio_util::sync::CancellationToken;

use crate::core::{ClickStreamItem, ClickStreamSource};

pub mod settings;

const MAX_CONNECT_RETRIES: u32 = 10;
const RETRY_DELAY: Duration = Duration::from_secs(3);

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

        // Retry consumer creation — the topic/partition may not be ready yet
        let mut stream = None;
        for attempt in 1..=MAX_CONNECT_RETRIES {
            let config = ConsumerConfigExtBuilder::default()
                .topic(settings.topic.clone())
                .offset_consumer(settings.consumer.clone())
                .offset_start(Offset::beginning())
                .offset_strategy(OffsetManagementStrategy::Auto)
                .offset_flush(Duration::from_millis(10000))
                .max_bytes(100000000)
                .build()
                .context("Failed to build consumer config")?;

            match self.fluvio.consumer_with_config(config).await {
                Ok(s) => {
                    info!(topic = %settings.topic, "Fluvio consumer connected");
                    stream = Some(s);
                    break;
                }
                Err(err) => {
                    if attempt == MAX_CONNECT_RETRIES {
                        return Err(err).context("Can not create fluvio hits consumer after retries");
                    }
                    warn!(
                        attempt,
                        max = MAX_CONNECT_RETRIES,
                        error = %err,
                        "Fluvio consumer not ready, retrying in {}s...",
                        RETRY_DELAY.as_secs()
                    );
                    tokio::time::sleep(RETRY_DELAY).await;
                }
            }
        }

        let mut stream = stream.expect("unreachable: loop guarantees stream or early return");

        while let Some(Ok(record)) = stream.next().await {
            let hit =
                serde_json::from_slice(record.as_ref()).context("Can not deserialize hit object")?;

            ts.send(hit).context("Can not re-send a hit to consumer")?;

            if token.is_cancelled() {
                break;
            }
        }

        // synchronously flush for shutdown (or none if intentionally ending processing)
        let _ = stream.offset_flush().await;

        Ok(())
    }
}
