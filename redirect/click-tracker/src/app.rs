use anyhow::Result;
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;
use typed_builder::TypedBuilder;

use crate::{
    adapters::HitStreamSourceType,
    core::{pipe::modules::clicks::ClickModules, tracking_pipe::TrackingPipe},
};

#[derive(TypedBuilder)]
#[builder(field_defaults(setter(prefix = "with_")))]
pub struct App {
    pipe: TrackingPipe<HitStreamSourceType, ClickModules>,
}

impl App {
    pub fn new(pipe: TrackingPipe<HitStreamSourceType, ClickModules>) -> Self {
        App { pipe }
    }

    pub async fn run(&self, token: CancellationToken) -> Result<JoinHandle<()>> {
        self.pipe.run(token).await
    }
}
