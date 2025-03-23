use aggregate::AggregateModule;
use anyhow::Result;

use init::InitModule;

use crate::core::{TrackingPipeContext, aggs::ClickAggsRegistrar, tracking_pipe::TrackingModule};

// pub mod enrich_location_module;
// pub mod enrich_session_module;
// pub mod enrich_user_agent_module;
pub mod aggregate;
pub mod init;

#[derive(Clone)]
pub enum ClickModules<R>
where
    R: ClickAggsRegistrar + Sync + Send + 'static,
{
    Init(InitModule),
    Aggregate(AggregateModule<R>),
}

#[async_trait::async_trait]
impl<R> TrackingModule for ClickModules<R>
where
    R: ClickAggsRegistrar + Sync + Send + 'static,
{
    async fn execute(&mut self, context: &mut TrackingPipeContext) -> Result<()> {
        match self {
            ClickModules::Init(module) => module.execute(context).await,
            ClickModules::Aggregate(module) => module.execute(context).await,
        }
    }
}
