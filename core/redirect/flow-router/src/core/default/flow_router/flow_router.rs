use crate::core::{
    base_flow_router::{FlowRouterResult, RedirectType, Result},
    BaseFlowRouter, BaseRoutesManager, PerRequestData,
};

use futures_util::Future;
use http::Request;
use moka::future::FutureExt;
use std::{pin::Pin, sync::Arc};

use super::{Middleware, MiddlewareNext};

#[derive(Clone, Debug)]
enum FlowStep {
    Initial,
    Start,
    UrlExtract,
    Register,
    BuildResult,
    End,
}

#[derive(Clone, Debug)]
struct FlowRouterContext {
    current_step: FlowStep,
}

#[derive(Clone, Debug)]
pub struct FlowRouter<RM>
where
    RM: BaseRoutesManager + Send + Sync + Clone + 'static,
{
    context: FlowRouterContext,
    routes_manager: Arc<RM>,
    modules: Arc<Vec<Box<dyn Middleware<FlowRouter<RM>>>>>,
}

impl<RM> FlowRouter<RM>
where
    RM: BaseRoutesManager + Send + Sync + Clone,
{
    pub fn new(routes_manager: RM, modules: Vec<Box<dyn Middleware<FlowRouter<RM>>>>) -> Self {
        Self {
            context: FlowRouterContext {
                current_step: FlowStep::Initial,
            },
            routes_manager: Arc::new(routes_manager),
            modules: Arc::new(modules),
        }
    }

    fn router_to<'a>(
        &'a mut self,
        step: FlowStep,
    ) -> Pin<Box<dyn Future<Output = Result<()>> + 'a + Send>> {
        self.context.current_step = step;

        match self.context.current_step {
            FlowStep::Start => self.handle_start(),
            FlowStep::UrlExtract => self.handle_url_extract(),
            FlowStep::Register => self.handle_register(),
            FlowStep::BuildResult => self.handle_build_result(),
            FlowStep::End => self.handle_end(),
            _ => panic!("Initial step set not allowed."),
        }
    }

    fn handle_start<'a>(&'a mut self) -> Pin<Box<dyn Future<Output = Result<()>> + 'a + Send>> {
        async move {
            println!("HandleStart");
            let _ = &self.handle_middleware().await;
            self.router_to(FlowStep::UrlExtract).await
        }
        .boxed()
    }

    fn handle_url_extract<'a>(
        &'a mut self,
    ) -> Pin<Box<dyn Future<Output = Result<()>> + 'a + Send>> {
        async move {
            println!("HandleUrlExtract");
            let _ = &self.handle_middleware().await;
            self.router_to(FlowStep::Register).await
        }
        .boxed()
    }

    fn handle_register<'a>(&'a mut self) -> Pin<Box<dyn Future<Output = Result<()>> + 'a + Send>> {
        async move {
            println!("HandleRegister");
            let _ = &self.handle_middleware().await;
            self.router_to(FlowStep::BuildResult).await
        }
        .boxed()
    }

    fn handle_build_result<'a>(
        &'a mut self,
    ) -> Pin<Box<dyn Future<Output = Result<()>> + 'a + Send>> {
        async move {
            println!("HandleBuildResult");
            let _ = &self.handle_middleware().await;
            self.router_to(FlowStep::End).await
        }
        .boxed()
    }

    fn handle_end<'a>(&'a mut self) -> Pin<Box<dyn Future<Output = Result<()>> + Send + 'a>> {
        async move {
            println!("HandleEnd");
            let _ = &self.handle_middleware().await;
            Ok(())
        }
        .boxed()
    }

    pub fn handle_middleware(&mut self) -> Pin<Box<dyn Future<Output = Result<()>> + Send>> {
        let chain = &mut self.modules.iter().map(|mw| mw.as_ref());

        let request_fn = |req: &Self| -> Pin<Box<dyn Future<Output = Result<()>> + Send>> {
            async move { Ok(()) }.boxed()
        };

        let request_fn = Box::new(request_fn);

        let next = MiddlewareNext { chain, request_fn };

        // // Run middleware chain
        next.handle(&self)
    }

    pub fn run<'a>(&'a mut self) -> Pin<Box<dyn Future<Output = Result<()>> + Send + 'a>> {
        self.router_to(FlowStep::Start)
    }
}

fn parse_domain_and_path<Req>(request: Request<Req>) -> (String, String)
where
    Req: Send + Sync,
{
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

impl<Req, RM> BaseFlowRouter<Req> for FlowRouter<RM>
where
    Req: Send + Sync,
    RM: BaseRoutesManager + Send + Sync + Clone,
{
    fn handle(
        &self,
        req: PerRequestData<Req>,
    ) -> impl std::future::Future<Output = Result<FlowRouterResult>> + Send {
        let (domain, path) = parse_domain_and_path(req.request);

        let routes_manager = Arc::new(self.routes_manager.clone());

        let fut = async move {
            let route = routes_manager
                .get_route("main", &domain, &path)
                .await
                .unwrap();

            let result = match route {
                Some(route) => {
                    let destination = route
                        .dest
                        .unwrap_or(String::from("Link destination is empty!"));

                    FlowRouterResult::Redirect(
                        destination.parse().unwrap(),
                        RedirectType::Temporary,
                    )
                }
                None => FlowRouterResult::Error,
            };

            Ok(result)
        }
        .boxed();

        fut
    }
}
