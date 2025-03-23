use anyhow::Result;
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;
use typed_builder::TypedBuilder;

use crate::{adapters::HitStreamSourceType, core::{pipe::modules::clicks::ClickModules, tracking_pipe::TrackingPipe, HitStreamSource}};

const BUFFER_SIZE: usize = 3;

#[derive(TypedBuilder)]
#[builder(field_defaults(setter(prefix = "with_")))]
pub struct App {
    pipe: TrackingPipe<HitStreamSourceType, ClickModules>
}

impl App {
    pub fn new(pipe: TrackingPipe<HitStreamSourceType, ClickModules>) -> Self {
        App { pipe }
    }

    pub async fn run(&self) -> Result<JoinHandle<()>> {
        let token: CancellationToken = CancellationToken::new();

        self.pipe.run().await
        // let (tx, rx) = std::sync::mpsc::sync_channel(BUFFER_SIZE);

        // for stream in &self.stream_sources {
        //     let tx = tx.clone();
        //     let token = token.clone();

        //     let _ = stream.pull(tx, token).await?;
        // }

        // let handler = tokio::spawn(async move {
        //     while let Ok(msg) = rx.recv() {
        //         //println!("received: {}", msg);
        //         if token.is_cancelled() {
        //             break;
        //         }
        //     }
        // });

    }
}
