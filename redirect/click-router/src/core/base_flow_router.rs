use anyhow::Result;
use dyn_clone::{clone_trait_object, DynClone};
use http::{Request, StatusCode, Uri};
use std::{
    self,
    collections::HashMap,
    fmt::{self, Display, Formatter, Result as FmtResult},
    net::SocketAddr,
};

use crate::{flow_router::{base_host_detector::HostInfo, base_ip_detector::IPInfo, base_protocol_detector::ProtoInfo}, model::Route};

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

#[derive(Clone, Debug)]
pub enum FlowStep {
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
    pub query: String
}


#[derive(Clone, Debug)]
pub struct FlowRouterContext {
    pub data: HashMap<&'static str, FlowRouterData>,
    pub current_step: FlowStep,
    pub host: Option<HostInfo>,
    pub client_ip: Option<IPInfo>,
    pub protocol: Option<ProtoInfo>,
    pub out_route: Option<Route>,
    pub in_route: FlowInRoute,
    pub request: PerRequestData,

    pub result: Option<FlowRouterResult>,
}

#[derive(Clone, Debug)]
pub struct PerConnHandler {
    pub local_addr: SocketAddr,
    pub remote_addr: SocketAddr,
    pub server_name: String,
    pub tls_info: Option<TlsInfo>,
}

#[derive(Clone, Debug)]
pub struct PerRequestData {
    pub local_addr: SocketAddr,
    pub remote_addr: SocketAddr,
    pub tls_info: Option<TlsInfo>,
    pub request: Request<()>,
}

#[derive(Clone, Debug)]
pub struct TlsInfo {
    pub sni_hostname: Option<String>,
    pub alpn_protocol: Option<String>,
    pub has_certificate: bool,
}
#[async_trait::async_trait(?Send)]
pub trait BaseFlowRouter: DynClone {
    async fn handle(&self, req: PerRequestData) -> Result<FlowRouterResult>;
}
clone_trait_object!(BaseFlowRouter);
