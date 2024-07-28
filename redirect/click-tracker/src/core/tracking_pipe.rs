use anyhow::Result;
use chrono::{DateTime, Utc};
use dyn_clone::{clone_trait_object, DynClone};
use std::{self, collections::HashMap};
use uuid::Uuid;

use crate::model::Hit;

use super::{
    location_detect::Country,
    user_agent_detect::{Device, UserAgent, OS},
    InitOnce,
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
pub struct TrackingPipeContext {
    pub id: Uuid,
    pub utc: DateTime<Utc>,
    pub hit: Hit,
    pub data: HashMap<&'static str, TrackingPipeData>,
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
            hit: hit,
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
    async fn start(&mut self) -> Result<()>;
}
