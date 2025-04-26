use anyhow::Result;
use conditional::ConditionalModule;
use not_found::NotFoundModule;
use redirect_only::RedirectOnlyModule;
use root::RootModule;

use super::{
    flow_module::{FlowModule, FlowStepContinuation},
    flow_router::{FlowRouter, FlowRouterContext},
};

pub mod redirect_only;
// pub mod abuse_module;
// pub mod full_path_module;
pub mod conditional;
pub mod not_found;
// pub mod open_graph_module;
// pub mod paused_module;
// pub mod robots_module;
pub mod root;

#[derive(Clone)]
pub enum FlowModules {
    Root(RootModule),
    Conditional(ConditionalModule),
    NotFound(NotFoundModule),
    RedirectOnly(RedirectOnlyModule),
}

#[async_trait::async_trait]
impl FlowModule for FlowModules {
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
