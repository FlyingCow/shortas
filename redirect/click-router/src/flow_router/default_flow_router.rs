use std::collections::HashMap;

use anyhow::Result;
use async_recursion::async_recursion;
use http::{uri::Scheme, Request, StatusCode};

use crate::{
    core::{
        base_flow_router::{
            FlowInRoute, FlowRouterContext, FlowRouterResult, FlowStep, PerRequestData,
            RedirectType,
        },
        BaseFlowRouter, BaseRoutesManager,
    },
    flow_router::base_flow_module::FlowStepContinuation,
    model::Route,
};

use super::{
    base_flow_module::BaseFlowModule, base_host_detector::BaseHostDetector,
    base_ip_detector::BaseIPDetector, base_protocol_detector::BaseProtocolDetector,
};

#[derive(Clone)]
pub struct DefaultFlowRouter {
    routes_manager: Box<dyn BaseRoutesManager>,
    host_detector: Box<dyn BaseHostDetector>,
    protocol_detector: Box<dyn BaseProtocolDetector>,
    ip_detector: Box<dyn BaseIPDetector>,
    modules: Vec<Box<dyn BaseFlowModule>>,
}

impl DefaultFlowRouter {
    pub fn new(
        routes_manager: Box<dyn BaseRoutesManager>,
        host_detector: Box<dyn BaseHostDetector>,
        protocol_detector: Box<dyn BaseProtocolDetector>,
        ip_detector: Box<dyn BaseIPDetector>,
        modules: Vec<Box<dyn BaseFlowModule>>,
    ) -> Self {
        DefaultFlowRouter {
            routes_manager,
            host_detector,
            protocol_detector,
            ip_detector,
            modules,
        }
    }

    #[async_recursion(?Send)]
    pub async fn router_to(&self, context: &mut FlowRouterContext, step: FlowStep) -> Result<()> {
        context.current_step = step;

        match context.current_step {
            FlowStep::Start => self.handle_start(context).await,
            FlowStep::UrlExtract => self.handle_url_extract(context).await,
            FlowStep::Register => self.handle_register(context).await,
            FlowStep::BuildResult => self.handle_build_result(context).await,
            FlowStep::End => self.handle_end(context).await,
            _ => panic!("Initial step set not allowed."),
        }
    }

    async fn handle_start(&self, context: &mut FlowRouterContext) -> Result<()> {
        println!("HandleStart");

        for module in &self.modules {
            let result = module.handle_start(context, &self).await?;

            if result == FlowStepContinuation::Break {
                return Ok(());
            }
        }

        self.router_to(context, FlowStep::UrlExtract).await
    }

    async fn handle_url_extract(&self, context: &mut FlowRouterContext) -> Result<()> {
        println!("HandleUrlExtract");

        for module in &self.modules {
            let result = module.handle_url_extract(context, &self).await?;

            if result == FlowStepContinuation::Break {
                return Ok(());
            }
        }

        self.router_to(context, FlowStep::Register).await
    }

    async fn handle_register(&self, context: &mut FlowRouterContext) -> Result<()> {
        println!("HandleRegister");

        for module in &self.modules {
            let result = module.handle_register(context, &self).await?;

            if result == FlowStepContinuation::Break {
                return Ok(());
            }
        }

        self.router_to(context, FlowStep::BuildResult).await
    }

    async fn handle_build_result(&self, context: &mut FlowRouterContext) -> Result<()> {
        println!("HandleBuildResult");

        for module in &self.modules {
            let result = module.handle_build_result(context, &self).await?;

            if result == FlowStepContinuation::Break {
                return Ok(());
            }
        }

        let result = match &context.out_route {
            Some(route) => {
                let destination = &route
                    .dest
                    .as_ref()
                    .unwrap_or(&String::from("http://test.com"))
                    .to_string();

                FlowRouterResult::Redirect(destination.parse().unwrap(), RedirectType::Temporary)
            }
            None => FlowRouterResult::Empty(StatusCode::NOT_FOUND),
        };

        context.result = Some(result);

        self.router_to(context, FlowStep::End).await
    }

    async fn handle_end(&self, context: &mut FlowRouterContext) -> Result<()> {
        println!("HandleEnd");

        for module in &self.modules {
            let result = module.handle_end(context, &self).await?;

            if result == FlowStepContinuation::Break {
                return Ok(());
            }
        }

        Ok(())
    }

    async fn get_route(&self, context: &FlowRouterContext) -> Result<Option<Route>> {
        let route = self
            .routes_manager
            .get_route(
                "main",
                context.in_route.host.as_str(),
                context.in_route.path.as_str(),
            )
            .await;

        Ok(route?)
    }

    async fn start(&self, request: PerRequestData) -> Result<FlowRouterContext> {
        let mut context = self.build_context(&request);

        for module in &self.modules {
            let result = module.init(&mut context, &self).await?;

            if result == FlowStepContinuation::Break {
                return Ok(context);
            }
        }

        if let None = context.out_route {
            context.out_route = self.get_route(&context).await?
        }

        self.router_to(&mut context, FlowStep::Start).await?;

        Ok(context)
    }

    fn build_route_uri(&self, request: &Request<()>) -> FlowInRoute {
        let path = &request.uri().path()[1..];

        let host_info = self.host_detector.detect(&request).unwrap();

        let query = request.uri().query().unwrap_or_default();

        let scheme = request.uri().scheme().unwrap_or(&Scheme::HTTP).to_string();

        let in_route = FlowInRoute {
            host: host_info.host,
            port: host_info.port,
            path: path.to_ascii_lowercase(),
            query: query.to_ascii_lowercase(),
            scheme: scheme.to_ascii_lowercase(),
        };

        in_route
    }

    fn build_context(&self, request: &PerRequestData) -> FlowRouterContext {
        let mut context = FlowRouterContext {
            data: HashMap::new(),
            current_step: FlowStep::Initial,
            in_route: self.build_route_uri(&request.request),
            client_ip: None,
            host: None,
            protocol: None,
            out_route: None, //self.get_route(&request).await?,
            result: None,
            request: request.clone(),
        };

        context.host = self.host_detector.detect(&context.request.request);
        context.protocol = self.protocol_detector.detect(&context.request.request);
        context.client_ip = self.ip_detector.detect(&context);

        context
    }
}

#[async_trait::async_trait(?Send)]
impl BaseFlowRouter for DefaultFlowRouter {
    async fn handle(&self, req: PerRequestData) -> Result<FlowRouterResult> {
        let context = self.start(req).await?;
        Ok(context.result.unwrap())
    }
}