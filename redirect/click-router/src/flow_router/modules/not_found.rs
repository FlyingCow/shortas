use anyhow::{Ok, Result};
use http::Uri;

use crate::{
    core::flow_router::{FlowRouterContext, FlowRouterResult, FlowStep, RedirectType},
    flow_router::{
        flow_module::{BaseFlowModule, FlowStepContinuation},
        default_flow_router::DefaultFlowRouter,
    },
};
const IS_404: &'static str = "is_404";

#[derive(Debug, Clone)]
pub struct NotFoundModule {}

#[async_trait::async_trait()]
impl BaseFlowModule for NotFoundModule {
    async fn handle_start(
        &self,
        context: &mut FlowRouterContext,
        flow_router: &DefaultFlowRouter,
    ) -> Result<FlowStepContinuation> {
        if let None = context.main_route {
            context.add_bool(IS_404, true);

            context.result = Some(FlowRouterResult::Redirect(
                Uri::from_static("https://notfound.com"),
                RedirectType::Temporary,
            ));

            flow_router.router_to(context, FlowStep::End).await?;

            return Ok(FlowStepContinuation::Break);
        }

        Ok(FlowStepContinuation::Continue)
    }

    async fn handle_end(
        &self,
        context: &mut FlowRouterContext,
        _flow_router: &DefaultFlowRouter,
    ) -> Result<FlowStepContinuation> {
        if context.is_data_true(IS_404) {
            return Ok(FlowStepContinuation::Break);
        } else {
            //println!("{}", "Is NOT 404");
        }

        Ok(FlowStepContinuation::Continue)
    }
}
