use std::ops::DerefMut;

use anyhow::Result;
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;

use super::{HitStreamSource, TrackingPipeContext};

const BUFFER_SIZE: usize = 3;

#[async_trait::async_trait]
pub trait TrackingModule {
    async fn execute(&mut self, _context: &mut TrackingPipeContext) -> Result<()>;
}

pub struct TrackingPipe<S, M>
where
    S: HitStreamSource,
    M: TrackingModule,
{
    stream_sources: Vec<S>,
    modules: Vec<M>,
}

impl<S, M> TrackingPipe<S, M>
where
    S: HitStreamSource,
    M: TrackingModule + Send + Sync + Clone + 'static,
{
    pub fn new(stream_sources: Vec<S>, modules: Vec<M>) -> Self {
        TrackingPipe {
            stream_sources,
            modules,
        }
    }

    pub async fn run(&self) -> Result<JoinHandle<()>> {
        let token: CancellationToken = CancellationToken::new();

        let (tx, rx) = std::sync::mpsc::sync_channel(BUFFER_SIZE);

        for stream in &self.stream_sources {
            let tx = tx.clone();
            let token = token.clone();

            let _ = stream.pull(tx, token).await?;
        }
        let mut modules = self.modules.clone();

        let handler = tokio::spawn(async move {
            while let Ok(hit) = rx.recv() {
                let mut context = TrackingPipeContext::new(hit);

                let mut modules = modules.deref_mut();
                for module in modules.deref_mut() {
                    let _result = module.execute(&mut context).await;
                }

                //println!("received: {}", msg);
                if token.is_cancelled() {
                    break;
                }
            }
        });

        Ok(handler)
    }
}
