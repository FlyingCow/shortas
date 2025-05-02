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
        context: &mut FlowRouterContext,
        flow_router: &FlowRouter,
    ) -> Result<FlowStepContinuation> {
        match self {
            FlowModules::Root(module) => module.init(context, flow_router).await,
            FlowModules::Conditional(module) => module.init(context, flow_router).await,
            FlowModules::NotFound(module) => module.init(context, flow_router).await,
            FlowModules::RedirectOnly(module) => module.init(context, flow_router).await,
        }
    }

    async fn handle_start(
        &self,
        context: &mut FlowRouterContext,
        flow_router: &FlowRouter,
    ) -> Result<FlowStepContinuation> {
        match self {
            FlowModules::Root(module) => module.handle_start(context, flow_router).await,
            FlowModules::Conditional(module) => module.handle_start(context, flow_router).await,
            FlowModules::NotFound(module) => module.handle_start(context, flow_router).await,
            FlowModules::RedirectOnly(module) => module.handle_start(context, flow_router).await,
        }
    }

    async fn handle_url_extract(
        &self,
        context: &mut FlowRouterContext,
        flow_router: &FlowRouter,
    ) -> Result<FlowStepContinuation> {
        match self {
            FlowModules::Root(module) => module.handle_url_extract(context, flow_router).await,
            FlowModules::Conditional(module) => {
                module.handle_url_extract(context, flow_router).await
            }
            FlowModules::NotFound(module) => module.handle_url_extract(context, flow_router).await,
            FlowModules::RedirectOnly(module) => {
                module.handle_url_extract(context, flow_router).await
            }
        }
    }

    async fn handle_register(
        &self,
        context: &mut FlowRouterContext,
        flow_router: &FlowRouter,
    ) -> Result<FlowStepContinuation> {
        match self {
            FlowModules::Root(module) => module.handle_register(context, flow_router).await,
            FlowModules::Conditional(module) => module.handle_register(context, flow_router).await,
            FlowModules::NotFound(module) => module.handle_register(context, flow_router).await,
            FlowModules::RedirectOnly(module) => module.handle_register(context, flow_router).await,
        }
    }

    async fn handle_build_result(
        &self,
        context: &mut FlowRouterContext,
        flow_router: &FlowRouter,
    ) -> Result<FlowStepContinuation> {
        match self {
            FlowModules::Root(module) => module.handle_build_result(context, flow_router).await,
            FlowModules::Conditional(module) => {
                module.handle_build_result(context, flow_router).await
            }
            FlowModules::NotFound(module) => module.handle_build_result(context, flow_router).await,
            FlowModules::RedirectOnly(module) => {
                module.handle_build_result(context, flow_router).await
            }
        }
    }

    async fn handle_end(
        &self,
        context: &mut FlowRouterContext,
        flow_router: &FlowRouter,
    ) -> Result<FlowStepContinuation> {
        match self {
            FlowModules::Root(module) => module.handle_end(context, flow_router).await,
            FlowModules::Conditional(module) => module.handle_end(context, flow_router).await,
            FlowModules::NotFound(module) => module.handle_end(context, flow_router).await,
            FlowModules::RedirectOnly(module) => module.handle_end(context, flow_router).await,
        }
    }
}
