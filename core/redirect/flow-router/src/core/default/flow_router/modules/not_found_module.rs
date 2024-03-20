use std::pin::Pin;

use anyhow::Result;
use futures_util::Future;

use crate::core::{default::{flow_router::{flow_router::{FlowRouterContext, FlowStep}, Middleware, MiddlewareNext}, FlowRouter}, BaseRoutesManager};

#[derive(Debug, Clone)]
pub struct NotFoundModule {}

impl <RM: BaseRoutesManager> Middleware<FlowRouter<RM>, FlowRouterContext> for NotFoundModule {
    fn handle(
        &self,
        router: &FlowRouter<RM>,
        context: &FlowRouterContext,
        next: MiddlewareNext<FlowRouter<RM>, FlowRouterContext>,
    ) -> Pin<Box<dyn Future<Output = Result<()>> + Send>> {

        println!("<{}:{}>", "NotFoundModule", context.current_step);
        // if context.request.request.uri().path_and_query().is_none() {
        //     return router.router_to(context, FlowStep::End);
        // }
            
        let result = next.handle(router, context);
        println!("</{}:{}>", "NotFoundModule", context.current_step);
        result
    }
}