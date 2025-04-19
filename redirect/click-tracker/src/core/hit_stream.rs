use std::sync::mpsc::Sender;

use anyhow::Result;
use dyn_clone::{clone_trait_object, DynClone};
use tokio_util::sync::CancellationToken;

use crate::model::Hit;

#[async_trait::async_trait]
pub trait BaseHitStream: DynClone {
    async fn pull(&mut self, tx: Sender<Hit>, cancelation_token: CancellationToken) -> Result<()>;
}

clone_trait_object!(BaseHitStream);
