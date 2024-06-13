use anyhow::Result;
use chrono::{DateTime, Utc};
use cookie::CookieJar;
use dyn_clone::{clone_trait_object, DynClone};
use http::{uri::Scheme, Extensions, HeaderMap, Method, StatusCode, Uri, Version};
use indexmap::IndexMap;
use multimap::MultiMap;
use once_cell::sync::OnceCell;
use uuid::Uuid;
use std::{
    self,
    collections::HashMap,
    fmt::{self, Display, Formatter, Result as FmtResult},
    net::SocketAddr,
};

use crate::{
    flow_router::{
        host_extract::HostInfo, ip_extract::IPInfo, language_extract::Language,
        protocol_extract::ProtoInfo,
    },
    model::Route,
};

use super::{
    location_detect::Country,
    user_agent_detect::{Device, UserAgent, OS},
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
    pub id: Uuid,
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
            id: Uuid::new_v4(),
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
#[async_trait::async_trait()]
pub trait BaseFlowRouter: DynClone {
    async fn handle(&self, req: RequestData, res: ResponseData) -> Result<FlowRouterResult>;
}
clone_trait_object!(BaseFlowRouter);
