use crate::{
    core::flow_router::FlowRouterContext,
    flow_router::{
        expression_evaluate::BaseExpressionEvaluator,
        flow_module::{BaseFlowModule, FlowStepContinuation},
        default_flow_router::DefaultFlowRouter,
    },
    model::route::RoutingPolicy,
};
use anyhow::Result;

const IS_CONDITIONAL: &'static str = "is_conditional";

#[derive(Clone)]
pub struct ConditionalModule {
    evaluator: Box<dyn BaseExpressionEvaluator + Send + Sync>,
}

impl ConditionalModule {
    pub fn new(evaluator: Box<dyn BaseExpressionEvaluator + Send + Sync>) -> Self {
        Self { evaluator }
    }
}

#[async_trait::async_trait()]
impl BaseFlowModule for ConditionalModule {
    async fn handle_start(
        &self,
        context: &mut FlowRouterContext,
        router: &DefaultFlowRouter,
    ) -> Result<FlowStepContinuation> {
        if context.main_route.is_none() {
            return Ok(FlowStepContinuation::Continue);
        }

        //preload heavy stuff if needed
        if let RoutingPolicy::Conditional(conditions) =
            &context.main_route.as_ref().unwrap().policy.clone()
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

            //println!("IS_CONDITIONAL");
            context.add_bool(IS_CONDITIONAL, true);
        }

        return Ok(FlowStepContinuation::Continue);
    }

    async fn handle_url_extract(
        &self,
        context: &mut FlowRouterContext,
        flow_router: &DefaultFlowRouter,
    ) -> Result<FlowStepContinuation> {
        if let RoutingPolicy::Conditional(conditions) = &context.main_route.as_ref().unwrap().policy
        {
            if let Some(matching) = &self.evaluator.find(context, conditions) {
                let out_route = flow_router
                    .get_route(matching.key.as_str(), context)
                    .await?;

                if let Some(route) = out_route {
                    context.out_route = Some(route);

                    return Ok(FlowStepContinuation::Continue);
                }
            }
        }

        return Ok(FlowStepContinuation::Continue);
    }
}
