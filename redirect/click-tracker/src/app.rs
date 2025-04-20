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

    pub async fn run(&self) -> Result<JoinHandle<()>> {
        let token: CancellationToken = CancellationToken::new();

        // Setup and execute subsystem tree
        // Toplevel::new(|s| async move {
        //     token.cancel();
        //     // s.start(SubsystemBuilder::new("Countdown", countdown_subsystem));
        // })
        // .catch_signals()
        // .handle_shutdown_requests(Duration::from_millis(1000))
        // .await
        // .map_err(Into::into);

        self.pipe.run(token).await
    }
}
