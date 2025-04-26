use anyhow::Result;
use async_recursion::async_recursion;
use chrono::{DateTime, Utc};
use cookie::CookieJar;
use http::{uri::Scheme, Extensions, HeaderMap, Method, StatusCode, Uri, Version};
use indexmap::IndexMap;
use multimap::MultiMap;
use once_cell::sync::OnceCell;
use std::{
    self,
    collections::HashMap,
    fmt::{self, Display, Formatter, Result as FmtResult},
    net::SocketAddr,
};
use ulid::Ulid;

use crate::{
    adapters::{HitRegistrarType, LocationDetectorType, UserAgentDetectorType},
    model::{
        hit::{Click, HitRoute},
        Hit, Route,
    },
};

use super::{
    flow_module::{FlowModule, FlowStepContinuation},
    hits_register::HitRegistrar,
    host::{HostExtractor, HostInfo},
    ip::{IPExtractor, IPInfo},
    language::{Language, LanguageExtractor},
    location::{Country, LocationDetector},
    modules::FlowModules,
    protocol::{ProtoInfo, ProtocolExtractor},
    routes::RoutesManager,
    user_agent::{Device, UserAgent, UserAgentDetector, OS},
    user_agent_string::UserAgentStringExtractor,
    user_settings::UserSettingsManager,
    InitOnce,
};

#[derive(Clone, Debug)]
pub enum RedirectType {
    Permanent,
    Temporary,
}

#[derive(Clone, Debug)]
pub enum FlowRouterResult {
    Empty(StatusCode),
    Json(String, StatusCode),
    PlainText(String, StatusCode),
    Proxied(Uri, StatusCode),
    Redirect(Uri, RedirectType),
    Retargeting(Uri, Vec<Uri>),
    Error,
}

impl Display for FlowRouterResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Default, Clone, Debug)]
pub enum FlowStep {
    #[default]
    Initial,
    Start,
    UrlExtract,
    Register,
    BuildResult,
    End,
}

impl Display for FlowStep {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{:?}", self)
    }
}

#[derive(Clone, Debug)]
pub enum FlowRouterData {
    Bool(bool),
    Number(f64),
    String(&'static str),
}

impl FlowRouterData {
    pub fn is_bool(&self, value: bool) -> bool {
        if let FlowRouterData::Bool(bool_value) = &self {
            return *bool_value == value;
        }

        false
    }

    pub fn is_string(&self, value: &str) -> bool {
        if let FlowRouterData::String(str_value) = &self {
            return value.eq_ignore_ascii_case(str_value);
        }

        false
    }

    pub fn is_num(&self, value: f64) -> bool {
        if let FlowRouterData::Number(num_value) = &self {
            return *num_value == value;
        }

        false
    }
}

impl FlowRouterContext {
    pub fn is_data_true(&self, bool_key: &'static str) -> bool {
        let data_value = self.data.get(&bool_key);

        if let Some(i) = data_value {
            return i.is_bool(true);
        }

        false
    }

    ///
    /// Adds a bool value to the context's data
    ///
    pub fn add_bool(&mut self, bool_key: &'static str, value: bool) {
        let _ = &self.data.insert(bool_key, FlowRouterData::Bool(value));
    }

    ///
    /// Adds a string value to the context's data
    ///
    pub fn add_string(&mut self, bool_key: &'static str, value: &'static str) {
        let _ = &self.data.insert(bool_key, FlowRouterData::String(value));
    }

    ///
    /// Adds a num value to the context's data
    ///
    pub fn add_num(&mut self, bool_key: &'static str, value: f64) {
        let _ = &self.data.insert(bool_key, FlowRouterData::Number(value));
    }
}

#[derive(Clone, Debug)]
pub struct FlowInRoute {
    pub scheme: String,
    pub host: String,
    pub port: u16,
    pub path: String,
    pub query: String,
}

impl FlowInRoute {
    pub fn new(scheme: String, host: String, port: u16, path: String, query: String) -> Self {
        Self {
            scheme,
            host,
            port,
            path,
            query,
        }
    }
}

#[derive(Debug)]
pub struct FlowRouterContext {
    pub id: String,
    pub utc: DateTime<Utc>,
    pub data: HashMap<&'static str, FlowRouterData>,
    pub client_os: InitOnce<Option<OS>>,
    pub client_ua: InitOnce<Option<UserAgent>>,
    pub client_device: InitOnce<Option<Device>>,
    pub client_country: InitOnce<Option<Country>>,
    pub current_step: FlowStep,
    pub host: Option<HostInfo>,
    pub client_ip: Option<IPInfo>,
    pub user_agent: Option<String>,
    pub client_langs: Option<Vec<Language>>,
    pub protocol: Option<ProtoInfo>,
    pub out_route: Option<Route>,
    pub main_route: Option<Route>,
    pub in_route: FlowInRoute,
    pub request: RequestData,
    pub response: ResponseData,

    pub result: Option<FlowRouterResult>,
}

impl FlowRouterContext {
    pub fn new(in_route: FlowInRoute, request: RequestData, response: ResponseData) -> Self {
        Self {
            id: Ulid::new().to_string(),
            utc: Utc::now(),
            data: HashMap::new(),
            client_os: InitOnce::default(None),
            client_ua: InitOnce::default(None),
            client_device: InitOnce::default(None),
            client_country: InitOnce::default(None),
            current_step: FlowStep::Initial,
            in_route,
            user_agent: None,
            client_ip: None,
            client_langs: None,
            host: None,
            protocol: None,
            out_route: None,
            main_route: None,
            result: None,
            request,
            response,
        }
    }
}

#[derive(Clone, Debug)]
pub struct PerConnHandler {
    pub local_addr: SocketAddr,
    pub remote_addr: SocketAddr,
    pub server_name: String,
    pub tls_info: Option<TlsInfo>,
}

#[derive(Clone, Debug, Default)]
pub struct ResponseData {
    /// The HTTP status code.WebTransportSession
    pub status_code: Option<StatusCode>,
    /// The HTTP headers.
    pub headers: HeaderMap,
    /// The HTTP version.
    pub version: Version,
    /// The HTTP cookies.
    pub cookies: CookieJar,
    /// Used to store extra data derived from the underlying protocol.
    pub extensions: Extensions,
}

#[derive(Clone, Debug, Default)]
pub struct RequestData {
    // The requested URL.
    pub uri: Uri,

    // The request headers.
    pub headers: HeaderMap,

    pub extensions: Extensions,

    // The request method.
    pub method: Method,

    pub cookies: CookieJar,

    pub params: IndexMap<String, String>,

    // accept: Option<Vec<Mime>>,
    pub queries: OnceCell<MultiMap<String, String>>,

    /// The version of the HTTP protocol used.
    pub version: Version,

    pub scheme: Option<Scheme>,

    pub local_addr: Option<SocketAddr>,

    pub remote_addr: Option<SocketAddr>,

    pub tls_info: Option<TlsInfo>,
}

#[derive(Clone, Debug)]
pub struct TlsInfo {
    pub sni_hostname: Option<String>,
    pub alpn_protocol: Option<String>,
    pub has_certificate: bool,
}

const MAIN_SWITCH: &'static str = "main";

pub struct FlowRouter {
    routes_manager: RoutesManager,
    settings_manager: UserSettingsManager,
    hit_registrar: HitRegistrarType,
    host_extractor: HostExtractor,
    protocol_extractor: ProtocolExtractor,
    ip_extractor: IPExtractor,
    user_agent_string_extractor: UserAgentStringExtractor,
    language_extractor: LanguageExtractor,
    user_agent_detector: UserAgentDetectorType,
    location_detector: LocationDetectorType,
    modules: Vec<FlowModules>,
}

impl FlowRouter {
    pub fn new(
        routes_manager: RoutesManager,
        settings_manager: UserSettingsManager,
        hit_registrar: HitRegistrarType,
        host_extractor: HostExtractor,
        protocol_extractor: ProtocolExtractor,
        ip_extractor: IPExtractor,
        user_agent_string_extractor: UserAgentStringExtractor,
        language_extractor: LanguageExtractor,
        user_agent_detector: UserAgentDetectorType,
        location_detector: LocationDetectorType,
        modules: Vec<FlowModules>,
    ) -> Self {
        FlowRouter {
            routes_manager,
            settings_manager,
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

        self.hit_registrar
            .register(Hit::click(
                context.id.clone(),
                context.utc,
                context.user_agent.clone(),
                Some(context.client_ip.clone().unwrap().address),
                Click::new(context.out_route.clone().unwrap().dest.unwrap()),
                HitRoute::from_route(&context.main_route),
            ))
            .await?;

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

        let _ = &self.replace_debug_data(&mut context);

        self.router_to(&mut context, FlowStep::Start).await?;

        Ok(context)
    }

    fn build_route_uri(&self, request: &RequestData) -> FlowInRoute {
        let path = &request.uri.path()[1..];

        let host_info = self.host_extractor.detect(&request, false).unwrap();

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

    fn allow_debug(&self, context: &mut FlowRouterContext) -> bool {
        if context.main_route.is_none() {
            return false;
        }

        let route = context.main_route.as_ref().unwrap();

        return route.properties.allow_debug;
    }

    pub fn load_country(&self, context: &mut FlowRouterContext) {
        if context.client_country.has_value() {
            return;
        }

        if context.client_ip.is_none() {
            context.client_country.init_with(None);
            return;
        }

        let country = self
            .location_detector
            .detect_country(&context.client_ip.clone().unwrap().address);

        context.client_country.init_with(country);
    }

    pub fn load_os(&self, context: &mut FlowRouterContext) {
        if context.client_os.has_value() {
            return;
        }

        if context.user_agent.is_none() {
            context.client_os.init_with(None);
            return;
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
            return;
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
            return;
        }

        let device = self
            .user_agent_detector
            .parse_device(context.user_agent.as_ref().unwrap());

        context.client_device.init_with(Some(device));
    }

    fn replace_debug_data(&self, context: &mut FlowRouterContext) {
        if !self.allow_debug(context) {
            return;
        }

        context.host = self.host_extractor.detect(&context.request, true);
        context.protocol = self.protocol_extractor.detect(&context.request, true);
        context.client_ip = self.ip_extractor.detect(&context.request, true);
        context.user_agent = self
            .user_agent_string_extractor
            .detect(&context.request, true);
        context.client_langs = self.language_extractor.detect(&context.request, true);
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

        context.host = self.host_extractor.detect(&context.request, false);
        context.protocol = self.protocol_extractor.detect(&context.request, false);
        context.client_ip = self.ip_extractor.detect(&context.request, false);
        context.user_agent = self
            .user_agent_string_extractor
            .detect(&context.request, false);
        context.client_langs = self.language_extractor.detect(&context.request, false);

        context
    }

    pub async fn handle(&self, req: RequestData, res: ResponseData) -> Result<FlowRouterResult> {
        let context = self.start(req, res).await?;
        Ok(context.result.unwrap())
    }
}
