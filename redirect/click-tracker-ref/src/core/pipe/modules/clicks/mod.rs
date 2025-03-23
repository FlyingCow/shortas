use anyhow::Result;

use init_module::InitModule;

use crate::core::{TrackingPipeContext, tracking_pipe::TrackingModule};

// pub mod enrich_location_module;
// pub mod enrich_session_module;
// pub mod enrich_user_agent_module;
pub mod init_module;
// pub mod register_aggregate_module;

#[derive(Clone)]
pub enum ClickModules {
    Init(InitModule),
}

#[async_trait::async_trait]
impl TrackingModule for ClickModules {
    async fn execute(&mut self, context: &mut TrackingPipeContext) -> Result<()> {
        match self {
            ClickModules::Init(module) => module.execute(context).await,
        }
    }
}
