use std::str::FromStr;

use anyhow::Result;
use http::Uri;

use crate::{
    core::base_flow_router::{FlowRouterContext, FlowRouterResult, RedirectType},
    flow_router::{
        base_flow_module::{BaseFlowModule, FlowStepContinuation},
        default_flow_router::DefaultFlowRouter,
    },
};

static IS_ROOT: &str = "is_root";

#[derive(Debug, Clone)]
pub struct RootModule {}

#[async_trait::async_trait(?Send)]
impl BaseFlowModule for RootModule {
    async fn handle_start(
        &self,
        context: &mut FlowRouterContext,
        _flow_router: &DefaultFlowRouter,
    ) -> Result<FlowStepContinuation> {
        
        if context.request.request.uri().path() == "/" {

            let root_uri = format!("https://{}", context.request.request.uri().host().unwrap());

            context.result = Some(FlowRouterResult::Redirect(
                Uri::from_str(&root_uri).unwrap(),
                RedirectType::Temporary,
            ));

            context.add_bool(IS_ROOT, true);

            println!("IS_ROOT");
            return Ok(FlowStepContinuation::Break);
        }

        Ok(FlowStepContinuation::Continue)
    }
}
