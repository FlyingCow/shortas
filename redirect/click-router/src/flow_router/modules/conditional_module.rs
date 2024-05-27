use crate::{
    core::base_flow_router::FlowRouterContext,
    flow_router::{
        base_flow_module::{BaseFlowModule, FlowStepContinuation},
        default_flow_router::DefaultFlowRouter,
    },
    model::route::RoutingPolicy,
};
use anyhow::Result;
use chrono::{prelude::*, DateTime, Utc};

const IS_CONDITIONAL: &'static str = "is_conditional";

#[derive(Clone)]
pub struct ConditionalModule {}

impl ConditionalModule {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait::async_trait(?Send)]
impl BaseFlowModule for ConditionalModule {
    async fn handle_start(
        &self,
        context: &mut FlowRouterContext,
        router: &DefaultFlowRouter,
    ) -> Result<FlowStepContinuation> {
        if context.out_route.is_none() {
            return Ok(FlowStepContinuation::Continue);
        }

        //preload heavy stuff if needed
        if let RoutingPolicy::Conditional(conditions) = &context.out_route.as_ref().unwrap().policy
        {
            if conditions
                .iter()
                .any(|condition| condition.condition.needs_browser())
            {
                router.load_browser(context);
            }
            if conditions
                .iter()
                .any(|condition| condition.condition.needs_os())
            {
                router.load_os(context);
            }
            if conditions
                .iter()
                .any(|condition| condition.condition.needs_device())
            {
                router.load_device(context);
            }
            if conditions
                .iter()
                .any(|condition| condition.condition.needs_country())
            {
                router.load_country(context);
            }
        }

        return Ok(FlowStepContinuation::Continue);
    }
}

pub struct ExpressionContext {}

impl ExpressionContext {
    fn get_os(&self, flow_context: &mut FlowRouterContext) -> Option<String> {
        Some("Windows".into())
    }
    fn get_ua(&self) -> Option<String> {
        Some("Chrome".into())
    }
    fn get_day_of_month(&self) -> Option<u32> {
        let now = Utc::now();
        Some(now.day())
    }
    fn get_day_of_week(&self) -> Option<u32> {
        let now = Utc::now();
        Some(now.weekday().num_days_from_sunday())
    }
    fn get_month(&self) -> Option<u32> {
        let now = Utc::now();
        Some(now.month())
    }
    fn get_date(&self) -> Option<DateTime<Utc>> {
        let now = Utc::now();
        Some(now)
    }
}
