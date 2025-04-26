use super::flow_router::FlowRouter;
use crate::core::flow_router::FlowRouterContext;
use anyhow::Result;

#[derive(PartialEq)]
pub enum FlowStepContinuation {
    Continue,
    Break,
}

#[async_trait::async_trait()]
pub trait FlowModule {
    async fn init(
        &self,
        _context: &mut FlowRouterContext,
        _flow_router: &FlowRouter,
    ) -> Result<FlowStepContinuation> {
        Ok(FlowStepContinuation::Continue)
    }

    async fn handle_start(
        &self,
        _context: &mut FlowRouterContext,
        _flow_router: &FlowRouter,
    ) -> Result<FlowStepContinuation> {
        Ok(FlowStepContinuation::Continue)
    }

    async fn handle_url_extract(
        &self,
        _context: &mut FlowRouterContext,
        _flow_router: &FlowRouter,
    ) -> Result<FlowStepContinuation> {
        Ok(FlowStepContinuation::Continue)
    }

    async fn handle_register(
        &self,
        _context: &mut FlowRouterContext,
        _flow_router: &FlowRouter,
    ) -> Result<FlowStepContinuation> {
        Ok(FlowStepContinuation::Continue)
    }

    async fn handle_build_result(
        &self,
        _context: &mut FlowRouterContext,
        _flow_router: &FlowRouter,
    ) -> Result<FlowStepContinuation> {
        Ok(FlowStepContinuation::Continue)
    }

    async fn handle_end(
        &self,
        _context: &mut FlowRouterContext,
        _flow_router: &FlowRouter,
    ) -> Result<FlowStepContinuation> {
        Ok(FlowStepContinuation::Continue)
    }
}
