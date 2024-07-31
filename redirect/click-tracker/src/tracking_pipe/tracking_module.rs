use anyhow::Result;
use dyn_clone::{clone_trait_object, DynClone};

use crate::core::tracking_pipe::TrackingPipeContext;


#[async_trait::async_trait()]
pub trait BaseTrackingModule: DynClone {
    async fn execute(&self, _context: &mut TrackingPipeContext) -> Result<()>;
}

clone_trait_object!(BaseTrackingModule);
