use anyhow::Result;
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;
use typed_builder::TypedBuilder;

use crate::{adapters::HitStreamSourceType, core::HitStreamSource};

const BUFFER_SIZE: usize = 3;

#[derive(TypedBuilder)]
#[builder(field_defaults(setter(prefix = "with_")))]
pub struct App {
    stream_sources: Vec<HitStreamSourceType>,
}

impl App {
    pub fn new(stream_sources: Vec<HitStreamSourceType>) -> Self {
        App { stream_sources }
    }

    pub async fn run(&self) -> Result<JoinHandle<()>> {
        let token: CancellationToken = CancellationToken::new();

        let (tx, rx) = std::sync::mpsc::sync_channel(BUFFER_SIZE);

        for stream in &self.stream_sources {
            let tx = tx.clone();
            let token = token.clone();

            let _ = stream.pull(tx, token).await?;
        }

        let handler = tokio::spawn(async move {
            while let Ok(msg) = rx.recv() {
                //println!("received: {}", msg);
                if token.is_cancelled() {
                    break;
                }
            }
        });

        Ok(handler)
    }
}
