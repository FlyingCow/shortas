use anyhow::Result;
use flume::Receiver;
use tokio_util::sync::CancellationToken;

use crate::core::ClickStreamItem;

use super::AggsPipeContext;

const BUFFER_SIZE: usize = 3;

#[async_trait::async_trait]
pub trait AggsModule {
    async fn execute(&mut self, _context: &mut AggsPipeContext) -> Result<()>;
}
pub struct AggsPipe<M>
where
    M: AggsModule,
{
    modules: Vec<M>,
}

impl<M> AggsPipe<M>
where
    M: AggsModule + Clone + 'static,
{
    pub fn new(modules: Vec<M>) -> Self {
        AggsPipe { modules }
    }

    pub async fn run(
        &self,
        _thread_id: usize,
        rx: Receiver<ClickStreamItem>,
        token: CancellationToken,
    ) {
        let mut modules = self.modules.clone();

        while let Ok(hit) = rx.recv() {
            // println!("{}", thread_id);
            let mut context = AggsPipeContext::new(hit);

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
