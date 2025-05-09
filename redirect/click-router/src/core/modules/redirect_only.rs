use anyhow::{Ok, Result};
use http::Method;

use crate::{
    core::{
        flow_module::{FlowModule, FlowStepContinuation},
        flow_router::{FlowRouter, FlowRouterContext, FlowStep, Request},
    },
    model::user_settings::SKIP_TRACKING,
};
const IS_REDIRECT_ONLY: &'static str = "is_redirect_only";

#[derive(Clone)]
pub struct RedirectOnlyModule {}

impl RedirectOnlyModule {
    pub fn new() -> Self {
        Self {}
    }

    async fn is_tracking_allowed(
        &self,
        flow_router: &FlowRouter,
        context: &mut FlowRouterContext<'_>,
    ) -> Result<bool> {
        if context
            .main_route
            .as_ref()
            .unwrap()
            .properties
            .owner_id
            .is_none()
        {
            return Ok(false);
        }

        let settings = &flow_router
            .get_user_settings(
                context
                    .main_route
                    .as_ref()
                    .unwrap()
                    .properties
                    .owner_id
                    .as_ref()
                    .unwrap(),
            )
            .await?;

        if let Some(settings) = settings {
            if settings.skip.contains(&SKIP_TRACKING.to_string()) {
                return Ok(false);
            }

            if settings.overflow {
                return Ok(false);
            }
        }

        Ok(true)
    }

    async fn is_redirect_only(
        &self,
        flow_router: &FlowRouter,
        context: &mut FlowRouterContext<'_>,
    ) -> Result<bool> {
        //no need to register,
        //since it will be handle by not found module
        if context.main_route.is_none() {
            return Ok(true);
        }

        if context.request.method() == Method::HEAD {
            return Ok(true);
        }

        if !self.is_tracking_allowed(&flow_router, context).await? {
            return Ok(true);
        }

        Ok(false)
    }
}

#[async_trait::async_trait()]
impl FlowModule for RedirectOnlyModule {
    async fn handle_start(
        &self,
        context: &mut FlowRouterContext,
        flow_router: &FlowRouter,
    ) -> Result<FlowStepContinuation> {
        if self.is_redirect_only(&flow_router, context).await? {
            context.add_bool(IS_REDIRECT_ONLY, true);
        }

        return Ok(FlowStepContinuation::Continue);
    }

    async fn handle_register(
        &self,
        context: &mut FlowRouterContext,
        flow_router: &FlowRouter,
    ) -> Result<FlowStepContinuation> {
        if context.is_data_true(IS_REDIRECT_ONLY) {
            flow_router
                .router_to(context, FlowStep::BuildResult)
                .await?;

            return Ok(FlowStepContinuation::Break);
        }

        Ok(FlowStepContinuation::Continue)
    }
}
