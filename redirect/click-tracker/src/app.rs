use std::{sync::OnceLock, thread::available_parallelism};

use crate::{
    adapters::HitStreamSourceType,
    core::{HitStreamSource, pipe::modules::clicks::ClickModules, tracking_pipe::TrackingPipe},
};
use anyhow::Result;
use flume::bounded;
use tokio::task::JoinSet;
use tokio_util::sync::CancellationToken;
use tracing::info;

static TRACKING_PIPE: OnceLock<TrackingPipe<ClickModules>> = OnceLock::new();

pub struct App;

impl App {
    pub async fn run(
        stream_sources: Vec<HitStreamSourceType>,
        pipe: TrackingPipe<ClickModules>,
        token: CancellationToken,
        channel_capacity: usize,
        parallelism: Option<usize>,
    ) -> Result<JoinSet<()>> {
        if TRACKING_PIPE.get().is_some() {
            panic!("Only one instance of app is allowed")
        }

        let mut set = JoinSet::new();

        let _ = TRACKING_PIPE.set(pipe);

        // Use configured parallelism or auto-detect (CPUs / 2)
        let parallelism = parallelism.unwrap_or_else(|| {
            usize::from(available_parallelism().unwrap()) / 2
        });

        info!("Pipeline configuration - parallelism: {}, channel_capacity: {}", parallelism, channel_capacity);

        let (tx, rx) = bounded(channel_capacity);

        for stream in stream_sources {
            let tx = tx.clone();
            let token = token.clone();

            set.spawn(async move {
                let _ = stream
                    .pull(tx, token)
                    .await
                    .expect("Can not start pulling from source stream");
            });
        }

        for n in 0..parallelism {
            let token = token.clone();
            let rx = rx.clone();

            info!("Starting thread: {}", n);

            set.spawn(TRACKING_PIPE.get().unwrap().run(n, rx.clone(), token));
        }

        Ok(set)
    }
}
