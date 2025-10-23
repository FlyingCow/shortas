use crate::{
    core::{
        expression::ExpressionEvaluator,
        flow_module::{FlowModule, FlowStepContinuation},
        flow_router::{FlowRouter, FlowRouterContext},
    },
    model::route::RoutingPolicy,
};
use anyhow::Result;

const IS_CONDITIONAL: &'static str = "is_conditional";

#[derive(Clone)]
pub struct ConditionalModule {
    evaluator: ExpressionEvaluator,
}

impl ConditionalModule {
    pub fn new() -> Self {
        Self {
            evaluator: ExpressionEvaluator::new(),
        }
    }
}

#[async_trait::async_trait()]
impl FlowModule for ConditionalModule {
    async fn handle_start(
        &self,
        context: &mut FlowRouterContext,
        router: &FlowRouter,
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
        flow_router: &FlowRouter,
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

#[cfg(test)]
mod tests {

    use crate::model::{
        expression::{DayOfMonth, Expression, OS, UA},
        route::{ConditionalRouting, RouteProperties},
        Route,
    };

    use super::*;

    #[test]
    fn should_be_serilizable() {
        let route: Route = Route {
            switch: "main".to_string(),
            link: "localhost%2ftest".to_string(),
            dest: Some("http://google.com".to_string()),
            properties: RouteProperties {
                route_id: Some("route_id".to_string()),
                domain_id: Some("route_id".to_string()),
                owner_id: Some("route_id".to_string()),
                creator_id: Some("route_id".to_string()),
                workspace_id: Some("route_id".to_string()),
                scripts: None,
                tags: None,
                custom: None,
                native: None,
                bundling: None,
                opengraph: false,
                allow_debug: true,
            },
            policy: RoutingPolicy::Conditional(
                [ConditionalRouting {
                    key: "test".to_string(),
                    condition: Expression {
                        ua: Some(UA::IN(
                            [
                                "Edge".to_string(),
                                "Chrome".to_string(),
                                "Firefox".to_string(),
                            ]
                            .to_vec(),
                        )),
                        day_of_month: Some(DayOfMonth::IN([1, 7, 10, 30].to_vec())),
                        and: Some(
                            [Box::new(Expression {
                                os: Some(OS::EQ("Windows".to_string())),
                                ..Default::default()
                            })]
                            .to_vec(),
                        ),
                        ..Default::default()
                    },
                }]
                .to_vec(),
            ),

            ..Default::default()
        };

        let serialized = serde_json::to_string(&route).unwrap().to_string();
        tracing::debug!("serialized = {}", serialized);
    }
}
