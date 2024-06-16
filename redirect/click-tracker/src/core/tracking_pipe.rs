use anyhow::Result;
use chrono::{DateTime, Utc};
use dyn_clone::{clone_trait_object, DynClone};
use std::{self, collections::HashMap, net::SocketAddr};
use uuid::Uuid;

use crate::model::Hit;

use super::{
    location_detect::Country,
    user_agent_detect::{Device, UserAgent, OS},
    InitOnce,
};

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

#[derive(Debug)]
pub struct TrackingPipeContext {
    pub id: Uuid,
    pub utc: DateTime<Utc>,
    pub data: HashMap<&'static str, FlowRouterData>,
    pub client_os: InitOnce<Option<OS>>,
    pub client_ua: InitOnce<Option<UserAgent>>,
    pub client_device: InitOnce<Option<Device>>,
    pub client_country: InitOnce<Option<Country>>,
}

impl TrackingPipeContext {
    pub fn new(hit: Hit) -> Self {
        Self {
            id: Uuid::new_v4(),
            utc: Utc::now(),
            data: HashMap::new(),
            client_os: InitOnce::default(None),
            client_ua: InitOnce::default(None),
            client_device: InitOnce::default(None),
            client_country: InitOnce::default(None),
        }
    }
}

impl TrackingPipeContext {
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
}

impl FlowRouterContext {
    pub fn new(hit: Hit) -> Self {
        Self {
            id: Uuid::new_v4(),
            utc: Utc::now(),
            data: HashMap::new(),
            client_os: InitOnce::default(None),
            client_ua: InitOnce::default(None),
            client_device: InitOnce::default(None),
            client_country: InitOnce::default(None),
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


#[derive(Clone, Debug)]
pub struct TlsInfo {
    pub sni_hostname: Option<String>,
    pub alpn_protocol: Option<String>,
    pub has_certificate: bool,
}
#[async_trait::async_trait()]
pub trait BaseTrackingPipe: DynClone {
    async fn handle(&self, hit: Hit) -> Result<()>;
}
clone_trait_object!(BaseTrackingPipe);
