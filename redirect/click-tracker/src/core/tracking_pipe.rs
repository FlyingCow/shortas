use anyhow::Result;
use flume::Receiver;
use tokio_util::sync::CancellationToken;

use crate::core::Hit;

use super::TrackingPipeContext;

#[async_trait::async_trait]
pub trait TrackingModule {
    async fn execute(&mut self, context: &mut TrackingPipeContext) -> Result<()>;
}

pub struct TrackingPipe<M>
where
    M: TrackingModule,
{
    modules: Vec<M>,
}

impl<M> TrackingPipe<M>
where
    M: TrackingModule + Clone + 'static,
{
    pub fn new(modules: Vec<M>) -> Self {
        TrackingPipe { modules }
    }

    pub async fn run(&self, _thread_id: usize, rx: Receiver<Hit>, token: CancellationToken) {
        let mut modules = self.modules.clone();

        while let Ok(hit) = rx.recv() {
            // println!("{}", thread_id);
            let mut context = TrackingPipeContext::new(hit);

            for module in modules.iter_mut() {
                let _result = module.execute(&mut context).await;
            }

            if token.is_cancelled() {
                tracing::info!("terminated!!!");
                break;
            }
        }
    }
}
