use std::collections::HashMap;

use anyhow::Result;
use async_recursion::async_recursion;
use chrono::Utc;
use http::{uri::Scheme, Request, StatusCode};

use crate::{
    core::{
        base_flow_router::{
            FlowInRoute, FlowRouterContext, FlowRouterResult, FlowStep, PerRequestData,
            RedirectType,
        },
        base_location_detector::BaseLocationDetector,
        base_user_agent_detector::BaseUserAgentDetector,
        BaseFlowRouter, BaseRoutesManager, InitOnce,
    },
    flow_router::base_flow_module::FlowStepContinuation,
    model::Route,
};

use super::{
    base_flow_module::BaseFlowModule, base_host_extractor::BaseHostExtractor,
    base_ip_extractor::BaseIPExtractor, base_language_extractor::BaseLanguageExtractor,
    base_protocol_extractor::BaseProtocolExtractor,
    base_user_agent_string_extractor::BaseUserAgentStringExtractor,
};

#[derive(Clone)]
pub struct DefaultFlowRouter {
    routes_manager: Box<dyn BaseRoutesManager>,
    host_extractor: Box<dyn BaseHostExtractor>,
    protocol_extractor: Box<dyn BaseProtocolExtractor>,
    ip_extractor: Box<dyn BaseIPExtractor>,
    user_agent_string_extractor: Box<dyn BaseUserAgentStringExtractor>,
    language_extractor: Box<dyn BaseLanguageExtractor>,
    user_agent_detector: Box<dyn BaseUserAgentDetector>,
    location_detector: Box<dyn BaseLocationDetector>,
    modules: Vec<Box<dyn BaseFlowModule>>,
}

impl DefaultFlowRouter {
    pub fn new(
        routes_manager: Box<dyn BaseRoutesManager>,
        host_extractor: Box<dyn BaseHostExtractor>,
        protocol_extractor: Box<dyn BaseProtocolExtractor>,
        ip_extractor: Box<dyn BaseIPExtractor>,
        user_agent_string_extractor: Box<dyn BaseUserAgentStringExtractor>,
        language_extractor: Box<dyn BaseLanguageExtractor>,
        user_agent_detector: Box<dyn BaseUserAgentDetector>,
        location_detector: Box<dyn BaseLocationDetector>,
        modules: Vec<Box<dyn BaseFlowModule>>,
    ) -> Self {
        DefaultFlowRouter {
            routes_manager,
            host_extractor,
            protocol_extractor,
            ip_extractor,
            user_agent_string_extractor,
            language_extractor,
            user_agent_detector,
            location_detector,
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

        let host_info = self.host_extractor.detect(&request).unwrap();

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

    pub fn load_country(&self, context: &FlowRouterContext) {
        if context.client_country.has_value() {
            return;
        }

        if context.client_ip.is_none() {
            context.client_os.init_with(None);
        }

        let country = self
            .location_detector
            .detect_country(&context.client_ip.as_ref().unwrap().address);

        context.client_country.init_with(country);
    }

    pub fn load_os(&self, context: &FlowRouterContext) {
        if context.client_os.has_value() {
            return;
        }

        if context.user_agent.is_none() {
            context.client_os.init_with(None);
        }

        let os = self
            .user_agent_detector
            .parse_os(context.user_agent.as_ref().unwrap());

        context.client_os.init_with(Some(os));
    }

    pub fn load_ua(&self, context: &FlowRouterContext) {
        if context.client_ua.has_value() {
            return;
        }

        if context.user_agent.is_none() {
            context.client_ua.init_with(None);
        }

        let ua = self
            .user_agent_detector
            .parse_user_agent(context.user_agent.as_ref().unwrap());

        context.client_ua.init_with(Some(ua));
    }

    pub fn load_device(&self, context: &FlowRouterContext) {
        if context.client_device.has_value() {
            return;
        }

        if context.user_agent.is_none() {
            context.client_device.init_with(None);
        }

        let device = self
            .user_agent_detector
            .parse_device(context.user_agent.as_ref().unwrap());

        context.client_device.init_with(Some(device));
    }

    fn build_context(&self, request: &PerRequestData) -> FlowRouterContext {
        let mut context = FlowRouterContext {
            utc: Utc::now(),
            data: HashMap::new(),
            client_os: InitOnce::default(None),
            client_ua: InitOnce::default(None),
            client_device: InitOnce::default(None),
            client_country: InitOnce::default(None),
            current_step: FlowStep::Initial,
            in_route: self.build_route_uri(&request.request),
            user_agent: None,
            client_ip: None,
            client_langs: None,
            host: None,
            protocol: None,
            out_route: None,
            result: None,
            request: request.clone(),
        };

        context.host = self.host_extractor.detect(&context.request.request);
        context.protocol = self.protocol_extractor.detect(&context.request.request);
        context.client_ip = self.ip_extractor.detect(&context);
        context.user_agent = self
            .user_agent_string_extractor
            .detect(&context.request.request);
        context.client_langs = self.language_extractor.detect(&context.request.request);

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
