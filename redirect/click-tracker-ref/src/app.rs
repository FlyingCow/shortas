use anyhow::Result;
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;
use typed_builder::TypedBuilder;

use crate::{
    adapters::HitStreamSourceType,
    core::{
        aggs::ClickAggsRegistrar, pipe::modules::clicks::ClickModules, tracking_pipe::TrackingPipe,
    },
};

#[derive(TypedBuilder)]
#[builder(field_defaults(setter(prefix = "with_")))]
pub struct App<A>
where
    A: ClickAggsRegistrar + Sync + Send + 'static,
{
    pipe: TrackingPipe<HitStreamSourceType, ClickModules<A>>,
}

impl<A> App<A>
where
    A: ClickAggsRegistrar + Sync + Send + Clone + 'static,
{
    pub fn new(pipe: TrackingPipe<HitStreamSourceType, ClickModules<A>>) -> Self {
        App { pipe }
    }

    pub async fn run(&self) -> Result<JoinHandle<()>> {
        let token: CancellationToken = CancellationToken::new();

        self.pipe.run(token).await
    }
}
