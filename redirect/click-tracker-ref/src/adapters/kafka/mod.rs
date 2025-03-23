use anyhow::Result;
use std::sync::mpsc::SyncSender;
use tokio::task::JoinHandle;

use tokio_util::sync::CancellationToken;

use crate::core::{Hit, HitStreamSource};

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
