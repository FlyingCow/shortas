use crate::{
    core::base_flow_router::FlowRouterContext,
    flow_router::{
        base_flow_module::{BaseFlowModule, FlowStepContinuation},
        default_flow_router::DefaultFlowRouter,
    },
    model::route::RoutingPolicy,
};
use anyhow::Result;

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
                .any(|routing| routing.condition.needs_ua())
            {
                router.load_ua(context);
            }
            if conditions
                .iter()
                .any(|routing| routing.condition.needs_os())
            {
                router.load_os(context);
            }
            if conditions
                .iter()
                .any(|routing| routing.condition.needs_device())
            {
                router.load_device(context);
            }
            if conditions
                .iter()
                .any(|routing| routing.condition.needs_country())
            {
                router.load_country(context);
            }

            println!("IS_CONDITIONAL");
            context.add_bool(IS_CONDITIONAL, true);
        }

        return Ok(FlowStepContinuation::Continue);
    }
}
