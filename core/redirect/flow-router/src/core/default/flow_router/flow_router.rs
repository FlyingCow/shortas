use std::{fmt, pin::Pin, sync::Arc};

use crate::{
    core::{
        base_flow_router::{FlowRouterResult, RedirectType, Result},
        BaseFlowRouter, BaseRoutesManager, PerRequestData,
    },
    domain::Route,
};

use futures_util::{Future, FutureExt};
use http::{Request, StatusCode};

use super::{Middleware, MiddlewareNext};

#[derive(Clone, Debug)]
pub enum FlowStep {
    Initial,
    Start,
    UrlExtract,
    Register,
    BuildResult,
    End,
}

impl fmt::Display for FlowStep {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Clone, Debug)]
pub struct FlowRouterContext {
    pub current_step: FlowStep,
    pub request: PerRequestData,
    pub current_route: Option<Route>,
    pub result: Option<FlowRouterResult>
}

#[derive(Clone, Debug)]
pub struct FlowRouter<RM>
where
    RM: BaseRoutesManager + Send + Sync + 'static,
{
    routes_manager: Arc<RM>,
    modules: Arc<Vec<Box<dyn Middleware<Self, FlowRouterContext>>>>,
}

impl<RM> FlowRouter<RM>
where
    RM: BaseRoutesManager + Send + Sync,
{
    pub fn new(
        routes_manager: RM,
        modules: Vec<Box<dyn Middleware<Self, FlowRouterContext>>>,
    ) -> Self {
        Self {
            routes_manager: Arc::new(routes_manager),
            modules: Arc::new(modules),
        }
    }

    pub fn router_to<'a>(
        &'a self,
        context: &'a mut FlowRouterContext,
        step: FlowStep,
    ) -> Pin<Box<dyn Future<Output = Result<()>> + Send + 'a>> {
        context.current_step = step;

        match context.current_step {
            FlowStep::Start => self.handle_start(context),
            FlowStep::UrlExtract => self.handle_url_extract(context),
            FlowStep::Register => self.handle_register(context),
            FlowStep::BuildResult => self.handle_build_result(context),
            FlowStep::End => self.handle_end(context),
            _ => panic!("Initial step set not allowed."),
        }
    }

    fn handle_start<'a>(
        &'a self,
        context: &'a mut FlowRouterContext,
    ) -> Pin<Box<dyn Future<Output = Result<()>> + Send + 'a>> {
        async move {
            println!("HandleStart");
            let _ = &self.handle_middleware(context).await;
            self.router_to(context, FlowStep::UrlExtract).await
        }
        .boxed()
    }

    fn handle_url_extract<'a>(
        &'a self,
        context: &'a mut FlowRouterContext,
    ) -> Pin<Box<dyn Future<Output = Result<()>> + Send + 'a>> {
        async move {
            println!("HandleUrlExtract");
            let _ = self.handle_middleware(context).await;
            self.router_to(context, FlowStep::Register).await
        }
        .boxed()
    }

    fn handle_register<'a>(
        &'a self,
        context: &'a mut FlowRouterContext,
    ) -> Pin<Box<dyn Future<Output = Result<()>> + Send + 'a>> {
        async move {
            println!("HandleRegister");
            let _ = &self.handle_middleware(context).await;
            self.router_to(context, FlowStep::BuildResult).await
        }
        .boxed()
    }

    fn handle_build_result<'a>(
        &'a self,
        context: &'a mut FlowRouterContext,
    ) -> Pin<Box<dyn Future<Output = Result<()>> + Send + 'a>> {


        async move {
            println!("HandleBuildResult");

            let result = match &context.current_route {
                Some(route) => {                
                    let destination = &route
                        .dest.as_ref()
                        .unwrap_or(&String::from("Link destination is empty!")).to_string();
    
                    FlowRouterResult::Redirect(
                        destination.parse().unwrap(),
                        RedirectType::Temporary,
                    )
                }
                None => FlowRouterResult::Empty(StatusCode::NOT_FOUND),
    
            };

            context.result = Some(result);
            
            let _ = &self.handle_middleware(context).await;
            self.router_to(context, FlowStep::End).await
        }
        .boxed()
    }

    fn handle_end<'a>(
        &'a self,
        context: &'a mut FlowRouterContext,
    ) -> Pin<Box<dyn Future<Output = Result<()>> + Send + 'a>> {
        async move {
            println!("HandleEnd");
            let _ = &self.handle_middleware(context).await;
            Ok(())
        }
        .boxed()
    }

    fn handle_middleware(
        &self,
        context: &mut FlowRouterContext,
    ) -> Pin<Box<dyn Future<Output = Result<()>> + Send>> {
        let chain = &mut self.modules.iter().map(|mw| mw.as_ref());

        let request_fn =
            |_: &Self, _: &FlowRouterContext| -> Pin<Box<dyn Future<Output = Result<()>> + Send>> {
                async move { Ok(()) }.boxed()
            };

        let request_fn = Box::new(request_fn);

        let next = MiddlewareNext { chain, request_fn };

        // // Run middleware chain
        next.handle(&self, context)
    }

    async fn get_router(&self, request: &PerRequestData) -> Result<Option<Route>> {
        let (domain, path) = parse_domain_and_path(&request.request);
        let route = self
            .routes_manager
            .get_route("main", &domain, &path)
            .await;

        Ok(route?)
    }

    async fn start(&self, request: PerRequestData) -> Result<FlowRouterContext> {
        let mut ctx = FlowRouterContext {
            result: None,
            current_step: FlowStep::Initial,
            current_route: self.get_router(&request).await?,
            request,
        };

        self.router_to(&mut ctx, FlowStep::Start).await?;

        Ok(ctx)
    }
}



fn parse_domain_and_path(request: &Request<()>) -> (String, String) {
    let path = &request.uri().path()[1..];

    let domain = match request.headers().get("Host") {
        Some(host) => host
            .to_str()
            .unwrap_or_default()
            .split(":")
            .next()
            .unwrap_or_default(),
        None => "",
    };

    (domain.to_ascii_lowercase(), path.to_ascii_lowercase())
}

impl<RM> BaseFlowRouter for FlowRouter<RM>
where
    RM: BaseRoutesManager + Send + Sync,
{
    fn handle(
        &self,
        request: PerRequestData,
    ) -> impl std::future::Future<Output = Result<FlowRouterResult>> + Send {

        let fut = async move {
            let context = &self.start(request).await?;

            Ok(context.result.clone().unwrap())
        }
        .boxed();

        fut
    }
}
