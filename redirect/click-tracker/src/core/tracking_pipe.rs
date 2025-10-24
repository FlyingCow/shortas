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

    pub async fn run(&self, thread_id: usize, rx: Receiver<Hit>, token: CancellationToken) {
        let mut modules = self.modules.clone();
        let mut events_processed = 0u64;
        let mut errors_count = 0u64;

        while let Ok(hit) = rx.recv() {
            let mut context = TrackingPipeContext::new(hit);

            // Execute modules with proper error handling
            for (idx, module) in modules.iter_mut().enumerate() {
                if let Err(e) = module.execute(&mut context).await {
                    errors_count += 1;
                    tracing::error!(
                        thread_id = thread_id,
                        module_idx = idx,
                        error = ?e,
                        "Module execution failed"
                    );
                    // Continue processing remaining modules even if one fails
                }
            }

            events_processed += 1;

            // Log progress periodically
            if events_processed % 1000 == 0 {
                tracing::info!(
                    thread_id = thread_id,
                    events_processed = events_processed,
                    errors = errors_count,
                    "Pipeline progress"
                );
            }

            if token.is_cancelled() {
                tracing::info!(
                    thread_id = thread_id,
                    events_processed = events_processed,
                    errors = errors_count,
                    "Pipeline terminated"
                );
                break;
            }
        }
    }
}
