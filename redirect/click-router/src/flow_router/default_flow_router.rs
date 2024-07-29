use std::collections::HashMap;

use anyhow::Result;
use async_recursion::async_recursion;
use chrono::Utc;
use http::{uri::Scheme, StatusCode};
use ulid::Ulid;

use crate::{
    core::{
        flow_router::{
            FlowInRoute, FlowRouterContext, FlowRouterResult, FlowStep, RedirectType, RequestData,
            ResponseData,
        },
        hits_register::BaseHitRegistrar,
        location_detect::BaseLocationDetector,
        user_agent_detect::BaseUserAgentDetector,
        BaseFlowRouter, BaseRoutesManager, InitOnce,
    },
    flow_router::flow_module::FlowStepContinuation,
    model::{Hit, Route},
};

use super::{
    flow_module::BaseFlowModule, host_extract::BaseHostExtractor, ip_extract::BaseIPExtractor,
    language_extract::BaseLanguageExtractor, protocol_extract::BaseProtocolExtractor,
    user_agent_string_extract::BaseUserAgentStringExtractor,
};

const MAIN_SWITCH: &'static str = "main";

#[derive(Clone)]
pub struct DefaultFlowRouter {
    routes_manager: Box<dyn BaseRoutesManager + Send + Sync>,
    hit_registrar: Box<dyn BaseHitRegistrar + Send + Sync>,
    host_extractor: Box<dyn BaseHostExtractor + Send + Sync>,
    protocol_extractor: Box<dyn BaseProtocolExtractor + Send + Sync>,
    ip_extractor: Box<dyn BaseIPExtractor + Send + Sync>,
    user_agent_string_extractor: Box<dyn BaseUserAgentStringExtractor + Send + Sync>,
    language_extractor: Box<dyn BaseLanguageExtractor + Send + Sync>,
    user_agent_detector: Box<dyn BaseUserAgentDetector + Send + Sync>,
    location_detector: Box<dyn BaseLocationDetector + Send + Sync>,
    modules: Vec<Box<dyn BaseFlowModule + Send + Sync>>,
}

impl DefaultFlowRouter {
    pub fn new(
        routes_manager: Box<dyn BaseRoutesManager + Send + Sync>,
        hit_registrar: Box<dyn BaseHitRegistrar + Send + Sync>,
        host_extractor: Box<dyn BaseHostExtractor + Send + Sync>,
        protocol_extractor: Box<dyn BaseProtocolExtractor + Send + Sync>,
        ip_extractor: Box<dyn BaseIPExtractor + Send + Sync>,
        user_agent_string_extractor: Box<dyn BaseUserAgentStringExtractor + Send + Sync>,
        language_extractor: Box<dyn BaseLanguageExtractor + Send + Sync>,
        user_agent_detector: Box<dyn BaseUserAgentDetector + Send + Sync>,
        location_detector: Box<dyn BaseLocationDetector + Send + Sync>,
        modules: Vec<Box<dyn BaseFlowModule + Send + Sync>>,
    ) -> Self {
        DefaultFlowRouter {
            routes_manager,
            hit_registrar,
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

    #[async_recursion()]
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
        for module in &self.modules {
            let result = module.handle_start(context, &self).await?;

            if result == FlowStepContinuation::Break {
                return Ok(());
            }
        }

        self.router_to(context, FlowStep::UrlExtract).await
    }

    async fn handle_url_extract(&self, context: &mut FlowRouterContext) -> Result<()> {
        for module in &self.modules {
            let result = module.handle_url_extract(context, &self).await?;

            if result == FlowStepContinuation::Break {
                return Ok(());
            }
        }

        self.router_to(context, FlowStep::Register).await
    }

    async fn handle_register(&self, context: &mut FlowRouterContext) -> Result<()> {
        for module in &self.modules {
            let result = module.handle_register(context, &self).await?;

            if result == FlowStepContinuation::Break {
                return Ok(());
            }
        }

        self.hit_registrar.register(Hit::new(
            context.id.clone().to_string(),
            Some(context.out_route.clone().unwrap().dest.unwrap()),
            context.user_agent.clone(),
            Some(context.client_ip.clone().unwrap().address)
        )).await?;

        self.router_to(context, FlowStep::BuildResult).await
    }

    async fn handle_build_result(&self, context: &mut FlowRouterContext) -> Result<()> {
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
        for module in &self.modules {
            let result = module.handle_end(context, &self).await?;

            if result == FlowStepContinuation::Break {
                return Ok(());
            }
        }

        Ok(())
    }

    pub async fn get_route(
        &self,
        switch: &str,
        context: &FlowRouterContext,
    ) -> Result<Option<Route>> {
        let route = self
            .routes_manager
            .get_route(
                switch,
                context.in_route.host.as_str(),
                context.in_route.path.as_str(),
            )
            .await;

        Ok(route?)
    }

    async fn start(&self, req: RequestData, res: ResponseData) -> Result<FlowRouterContext> {
        let mut context = self.build_context(&req, &res);

        for module in &self.modules {
            let result = module.init(&mut context, &self).await?;

            if result == FlowStepContinuation::Break {
                return Ok(context);
            }
        }

        if let None = context.main_route {
            context.main_route = self.get_route(MAIN_SWITCH, &context).await?;
            context.out_route = context.main_route.clone();
        }

        self.router_to(&mut context, FlowStep::Start).await?;

        Ok(context)
    }

    fn build_route_uri(&self, request: &RequestData) -> FlowInRoute {
        let path = &request.uri.path()[1..];

        let host_info = self.host_extractor.detect(&request).unwrap();

        let query = request.uri.query().unwrap_or_default();

        let scheme = request.uri.scheme().unwrap_or(&Scheme::HTTP).to_string();

        let in_route = FlowInRoute {
            host: host_info.host,
            port: host_info.port,
            path: path.to_ascii_lowercase(),
            query: query.to_ascii_lowercase(),
            scheme: scheme.to_ascii_lowercase(),
        };

        in_route
    }

    pub fn load_country(&self, context: &mut FlowRouterContext) {
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

    pub fn load_os(&self, context: &mut FlowRouterContext) {
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

    pub fn load_ua(&self, context: &mut FlowRouterContext) {
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

    pub fn load_device(&self, context: &mut FlowRouterContext) {
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

    fn build_context(&self, req: &RequestData, res: &ResponseData) -> FlowRouterContext {
        let mut context = FlowRouterContext {
            id: Ulid::new().to_string(),
            utc: Utc::now(),
            data: HashMap::new(),
            client_os: InitOnce::default(None),
            client_ua: InitOnce::default(None),
            client_device: InitOnce::default(None),
            client_country: InitOnce::default(None),
            current_step: FlowStep::Initial,
            in_route: self.build_route_uri(req),
            user_agent: None,
            client_ip: None,
            client_langs: None,
            host: None,
            protocol: None,
            out_route: None,
            main_route: None,
            result: None,
            request: req.clone(),
            response: res.clone(),
        };

        context.host = self.host_extractor.detect(&context.request);
        context.protocol = self.protocol_extractor.detect(&context.request);
        context.client_ip = self.ip_extractor.detect(&context.request);
        context.user_agent = self.user_agent_string_extractor.detect(&context.request);
        context.client_langs = self.language_extractor.detect(&context.request);

        context
    }
}

#[async_trait::async_trait()]
impl BaseFlowRouter for DefaultFlowRouter {
    async fn handle(&self, req: RequestData, res: ResponseData) -> Result<FlowRouterResult> {
        let context = self.start(req, res).await?;
        Ok(context.result.unwrap())
    }
}
