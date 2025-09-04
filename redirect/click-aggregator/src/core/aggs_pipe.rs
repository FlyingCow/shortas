use std::ops::DerefMut;

use anyhow::Result;
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;
use typed_builder::TypedBuilder;

use super::{AggsPipeContext, ClickStreamSource};

const BUFFER_SIZE: usize = 3;

#[async_trait::async_trait]
pub trait AggsModule {
    async fn execute(&mut self, _context: &mut AggsPipeContext) -> Result<()>;
}

#[derive(TypedBuilder)]
#[builder(field_defaults(setter(prefix = "with_")))]
pub struct AggsPipe<S, M>
where
    S: ClickStreamSource,
    M: AggsModule,
{
    stream_sources: Vec<S>,
    modules: Vec<M>,
}

impl<S, M> AggsPipe<S, M>
where
    S: ClickStreamSource,
    M: AggsModule + Send + Sync + Clone + 'static,
{
    pub fn new(stream_sources: Vec<S>, modules: Vec<M>) -> Self {
        AggsPipe {
            stream_sources,
            modules,
        }
    }

    pub async fn run(&self, token: CancellationToken) -> Result<JoinHandle<()>> {
        let (tx, rx) = std::sync::mpsc::sync_channel(BUFFER_SIZE);

        for stream in &self.stream_sources {
            let tx = tx.clone();
            let token = token.clone();

            let _ = stream.pull(tx, token).await?;
        }
        let mut modules = self.modules.clone();

        let handler = tokio::spawn(async move {
            while let Ok(hit) = rx.recv() {
                let mut context = AggsPipeContext::new(hit);

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
