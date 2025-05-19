use anyhow::Result;
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;
use typed_builder::TypedBuilder;

use crate::{
    adapters::ClickStreamSourceType,
    core::{aggs_pipe::AggsPipe, pipe::modules::clicks::AggsModules},
};

#[derive(TypedBuilder)]
#[builder(field_defaults(setter(prefix = "with_")))]
pub struct App {
    pipe: AggsPipe<ClickStreamSourceType, AggsModules>,
}

impl App {
    pub fn new(pipe: AggsPipe<ClickStreamSourceType, AggsModules>) -> Self {
        App { pipe }
    }

    pub async fn run(&self, token: CancellationToken) -> Result<JoinHandle<()>> {
        self.pipe.run(token).await
    }
}
