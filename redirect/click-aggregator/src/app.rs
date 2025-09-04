use std::{sync::OnceLock, thread::available_parallelism};

use anyhow::Result;
use flume::bounded;
use tokio::task::JoinSet;
use tokio_util::sync::CancellationToken;
use tracing::info;

use crate::{
    adapters::ClickStreamSourceType,
    core::{aggs_pipe::AggsPipe, pipe::modules::clicks::AggsModules, ClickStreamSource},
};

static AGGS_PIPE: OnceLock<AggsPipe<AggsModules>> = OnceLock::new();

pub struct App;

impl App {
    pub async fn run(
        stream_sources: Vec<ClickStreamSourceType>,
        pipe: AggsPipe<AggsModules>,
        token: CancellationToken,
    ) -> Result<JoinSet<()>> {
        if AGGS_PIPE.get().is_some() {
            panic!("Only one instance of app is allowed")
        }

        let mut set = JoinSet::new();

        let _ = AGGS_PIPE.set(pipe);

        let parallelism = usize::from(available_parallelism().unwrap()) / 2;

        let (tx, rx) = bounded(parallelism);

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

            set.spawn(AGGS_PIPE.get().unwrap().run(n, rx.clone(), token));
        }

        Ok(set)
    }
}
