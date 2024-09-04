use anyhow::Result;
use chrono::{DateTime, Utc};
use std::{self, collections::HashMap};
use tokio_util::sync::CancellationToken;
use ulid::Ulid;

use crate::model::Hit;

use super::{
    location_detect::Country, session_detect::Session, user_agent_detect::{Device, UserAgent, OS}
};

#[derive(Clone, Debug)]
pub enum TrackingPipeData {
    Bool(bool),
    Number(f64),
    String(&'static str),
}

impl TrackingPipeData {
    pub fn is_bool(&self, value: bool) -> bool {
        if let TrackingPipeData::Bool(bool_value) = &self {
            return *bool_value == value;
        }

        false
    }

    pub fn is_string(&self, value: &str) -> bool {
        if let TrackingPipeData::String(str_value) = &self {
            return value.eq_ignore_ascii_case(str_value);
        }

        false
    }

    pub fn is_num(&self, value: f64) -> bool {
        if let TrackingPipeData::Number(num_value) = &self {
            return *num_value == value;
        }

        false
    }
}

#[derive(Debug)]
pub struct TrackingError {
    pub utc: DateTime<Utc>,
    pub tries: u16,
    pub error: String,
    pub stack_trace: String 
}

#[derive(Debug)]
pub enum TrackingState {
    Ok,
    Error(TrackingError),
}

#[derive(Debug)]
pub struct TrackingPipeContext {
    pub id: String,
    pub utc: DateTime<Utc>,
    pub hit: Hit,
    pub data: HashMap<&'static str, TrackingPipeData>,
    pub client_os: Option<OS>,
    pub client_ua: Option<UserAgent>,
    pub client_device: Option<Device>,
    pub client_country: Option<Country>,
    pub spider: bool,
    pub session: Option<Session>,
    pub state: TrackingState,
}

impl TrackingPipeContext {
    pub fn new(hit: Hit) -> Self {
        Self {
            id: Ulid::new().to_string(),
            utc: Utc::now(),
            hit: hit,
            data: HashMap::new(),
            client_os: None,
            client_ua: None,
            client_device: None,
            client_country: None,
            session: None,
            state: TrackingState::Ok,
            spider: false
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
        let _ = &self.data.insert(bool_key, TrackingPipeData::Bool(value));
    }

    ///
    /// Adds a string value to the context's data
    ///
    pub fn add_string(&mut self, bool_key: &'static str, value: &'static str) {
        let _ = &self.data.insert(bool_key, TrackingPipeData::String(value));
    }

    ///
    /// Adds a num value to the context's data
    ///
    pub fn add_num(&mut self, bool_key: &'static str, value: f64) {
        let _ = &self.data.insert(bool_key, TrackingPipeData::Number(value));
    }
}

#[async_trait::async_trait()]
pub trait BaseTrackingPipe {
    async fn start(&mut self, cnacelation_token: CancellationToken) -> Result<()>;
}
