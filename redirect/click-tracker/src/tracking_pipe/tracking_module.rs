use anyhow::Result;
use dyn_clone::{clone_trait_object, DynClone};

use crate::core::tracking_pipe::TrackingPipeContext;

#[derive(PartialEq)]
pub enum FlowStepContinuation {
    Continue,
    Break
}

#[async_trait::async_trait()]
pub trait BaseTrackingModule: DynClone {

    async fn init(
        &self,
        _context: &mut TrackingPipeContext,
    ) -> Result<FlowStepContinuation>{
        Ok(FlowStepContinuation::Continue)
    }

    async fn handle_start(
        &self,
        _context: &mut TrackingPipeContext,
    ) -> Result<FlowStepContinuation>{
        Ok(FlowStepContinuation::Continue)
    }

    async fn execute(
        &self,
        _context: &mut TrackingPipeContext,
    ) -> Result<FlowStepContinuation>{
        Ok(FlowStepContinuation::Continue)
    }
}

clone_trait_object!(BaseTrackingModule);
