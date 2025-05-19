use anyhow::Result;
use store::StoreModule;

use init::InitModule;

use crate::core::{aggs_pipe::AggsModule, AggsPipeContext};

pub mod init;
pub mod store;

#[derive(Clone)]
pub enum AggsModules {
    Init(InitModule),
    Store(StoreModule),
}

#[async_trait::async_trait]
impl AggsModule for AggsModules {
    async fn execute(&mut self, context: &mut AggsPipeContext) -> Result<()> {
        match self {
            AggsModules::Init(module) => module.execute(context).await,
            AggsModules::Store(module) => module.execute(context).await,
        }
    }
}
