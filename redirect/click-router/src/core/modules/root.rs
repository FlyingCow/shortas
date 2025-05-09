use std::str::FromStr;

use anyhow::Result;
use http::Uri;

use crate::core::{
    flow_module::{FlowModule, FlowStepContinuation},
    flow_router::{FlowRouter, FlowRouterContext, FlowRouterResult, RedirectType, Request},
};

static IS_ROOT: &str = "is_root";

#[derive(Debug, Clone)]
pub struct RootModule {}

impl RootModule {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait::async_trait()]
impl FlowModule for RootModule {
    async fn handle_start(
        &self,
        context: &mut FlowRouterContext,
        _flow_router: &FlowRouter,
    ) -> Result<FlowStepContinuation> {
        if context.request.uri().path() == "/" {
            let root_uri = format!("https://{}", context.request.uri().host().unwrap());

            context.result = Some(FlowRouterResult::Redirect(
                Uri::from_str(&root_uri).unwrap(),
                RedirectType::Temporary,
            ));

            context.add_bool(IS_ROOT, true);

            //println!("IS_ROOT");
            return Ok(FlowStepContinuation::Break);
        }

        Ok(FlowStepContinuation::Continue)
    }
}
