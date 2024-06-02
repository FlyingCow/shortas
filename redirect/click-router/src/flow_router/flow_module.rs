use anyhow::Result;
use dyn_clone::{clone_trait_object, DynClone};

use crate::core::flow_router::FlowRouterContext;

use super::default_flow_router::DefaultFlowRouter;

#[derive(PartialEq)]
pub enum FlowStepContinuation {
    Continue,
    Break
}

#[async_trait::async_trait()]
pub trait BaseFlowModule: DynClone {

    async fn init(
        &self,
        _context: &mut FlowRouterContext,
        _flow_router: &DefaultFlowRouter,
    ) -> Result<FlowStepContinuation>{
        Ok(FlowStepContinuation::Continue)
    }

    async fn handle_start(
        &self,
        _context: &mut FlowRouterContext,
        _flow_router: &DefaultFlowRouter,
    ) -> Result<FlowStepContinuation>{
        Ok(FlowStepContinuation::Continue)
    }

    async fn handle_url_extract(
        &self,
        _context: &mut FlowRouterContext,
        _flow_router: &DefaultFlowRouter,
    ) -> Result<FlowStepContinuation>{
        Ok(FlowStepContinuation::Continue)
    }

    async fn handle_register(
        &self,
        _context: &mut FlowRouterContext,
        _flow_router: &DefaultFlowRouter,
    ) -> Result<FlowStepContinuation>{
        Ok(FlowStepContinuation::Continue)
    }

    async fn handle_build_result(
        &self,
        _context: &mut FlowRouterContext,
        _flow_router: &DefaultFlowRouter,
    ) -> Result<FlowStepContinuation>{
        Ok(FlowStepContinuation::Continue)
    }

    async fn handle_end(
        &self,
        _context: &mut FlowRouterContext,
        _flow_router: &DefaultFlowRouter,
    ) -> Result<FlowStepContinuation>{
        Ok(FlowStepContinuation::Continue)
    }
}

clone_trait_object!(BaseFlowModule);
