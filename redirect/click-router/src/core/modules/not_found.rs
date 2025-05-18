use std::str::FromStr;

use anyhow::{Ok, Result};
use http::{StatusCode, Uri};
use string_format::*;

use crate::{
    core::{
        flow_module::{FlowModule, FlowStepContinuation},
        flow_router::{
            FlowRouter, FlowRouterContext, FlowRouterResult, FlowStep, RedirectType, Request,
        },
    },
    settings::Redirect,
};

const IS_404: &'static str = "is_404";

#[derive(Debug, Clone)]
pub struct NotFoundModule {
    redirect: Redirect,
}

impl NotFoundModule {
    pub fn new(redirect: Redirect) -> Self {
        Self { redirect }
    }
}

#[async_trait::async_trait()]
impl FlowModule for NotFoundModule {
    async fn handle_start(
        &self,
        context: &mut FlowRouterContext,
        flow_router: &FlowRouter,
    ) -> Result<FlowStepContinuation> {
        if let None = context.main_route {
            context.add_bool(IS_404, true);

            let not_found_uri = string_format!(
                self.redirect.not_found_url.clone(),
                context.request.uri().host().unwrap().to_string()
            );

            context.result = Some(FlowRouterResult::Proxied(
                Uri::from_str(&not_found_uri).unwrap(),
                StatusCode::OK,
            ));

            flow_router.router_to(context, FlowStep::End).await?;

            return Ok(FlowStepContinuation::Break);
        }

        Ok(FlowStepContinuation::Continue)
    }

    async fn handle_end(
        &self,
        context: &mut FlowRouterContext,
        _flow_router: &FlowRouter,
    ) -> Result<FlowStepContinuation> {
        if context.is_data_true(IS_404) {
            return Ok(FlowStepContinuation::Break);
        } else {
            //println!("{}", "Is NOT 404");
        }

        Ok(FlowStepContinuation::Continue)
    }
}
