use aggregate::AggregateModule;
use anyhow::Result;

use init::InitModule;
use location::EnrichLocationModule;
use session::EnrichSessionModule;
use user_agent::EnrichUserAgentModule;

use crate::core::{TrackingPipeContext, tracking_pipe::TrackingModule};

pub mod aggregate;
pub mod init;
pub mod location;
pub mod session;
pub mod user_agent;

#[derive(Clone)]
pub enum ClickModules {
    Init(InitModule),
    Aggregate(AggregateModule),
    Location(EnrichLocationModule),
    Session(EnrichSessionModule),
    UserAgent(EnrichUserAgentModule),
}

#[async_trait::async_trait]
impl TrackingModule for ClickModules {
    async fn execute(&mut self, context: &mut TrackingPipeContext) -> Result<()> {
        match self {
            ClickModules::Init(module) => module.execute(context).await,
            ClickModules::Aggregate(module) => module.execute(context).await,
            ClickModules::Location(module) => module.execute(context).await,
            ClickModules::Session(module) => module.execute(context).await,
            ClickModules::UserAgent(module) => module.execute(context).await,
        }
    }
}
