use anyhow::Result;
use std::sync::mpsc::{self, RecvError};
use std::sync::mpsc::{Receiver, Sender};
use tokio_util::sync::CancellationToken;
use tracing::{error, warn};

use crate::core::{
    hit_stream::BaseHitStream,
    tracking_pipe::{BaseTrackingPipe, TrackingPipeContext},
};
use crate::model::Hit;

use super::tracking_module::BaseTrackingModule;

pub struct DefaultTrackingPipe {
    hit_stream: Box<dyn BaseHitStream + Send + Sync + 'static>,
    modules: Vec<Box<dyn BaseTrackingModule + Send + Sync + 'static>>,
}

impl DefaultTrackingPipe {
    pub fn new(
        hit_stream: Box<dyn BaseHitStream + Send + Sync + 'static>,
        modules: Vec<Box<dyn BaseTrackingModule + Send + Sync>>,
    ) -> Self {
        DefaultTrackingPipe {
            hit_stream: hit_stream,
            modules,
        }
    }
}

#[async_trait::async_trait()]
impl BaseTrackingPipe for DefaultTrackingPipe {
    async fn start(&mut self, cancelation_token: CancellationToken) -> Result<()> {
        let mut hit_stream = self.hit_stream.clone();
        let mut modules = self.modules.clone();
        let (tx, rx): (Sender<Hit>, Receiver<Hit>) = mpsc::channel();

        hit_stream
            .as_mut()
            .pull(tx, cancelation_token)
            .await
            .expect("Can not start hit stream consumer");

        let handler = tokio::spawn(async move {
            loop {
                match rx.recv() {
                    Err(error) => {
                        error!("Unrecoverable error! Stopping Tracking pipe: {}", error);
                        break;
                    }
                    Ok(hit) => {
                        let mut context = TrackingPipeContext::new(hit);
                        for module in &mut modules {
                            let _result = module.execute(&mut context).await;
                        }
                    }
                };
            }
        });
        handler.await.map_err(anyhow::Error::from)
    }
}
