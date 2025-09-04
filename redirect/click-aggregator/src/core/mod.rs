use std::collections::HashMap;

use chrono::DateTime;
use chrono::Utc;
use serde::{Deserialize, Serialize};

pub mod aggs;
pub mod aggs_pipe;
pub mod click_stream;
pub mod pipe;

pub use click_stream::ClickStreamSource;
use ulid::Ulid;

#[derive(Clone, Debug)]
pub enum AggsPipeData {
    Bool(bool),
    Number(f64),
    String(&'static str),
}

impl AggsPipeData {
    pub fn is_bool(&self, value: bool) -> bool {
        if let AggsPipeData::Bool(bool_value) = &self {
            return *bool_value == value;
        }

        false
    }

    pub fn is_string(&self, value: &str) -> bool {
        if let AggsPipeData::String(str_value) = &self {
            return value.eq_ignore_ascii_case(str_value);
        }

        false
    }

    pub fn is_num(&self, value: f64) -> bool {
        if let AggsPipeData::Number(num_value) = &self {
            return *num_value == value;
        }

        false
    }
}

#[derive(Debug)]
pub struct AggsError {
    pub utc: DateTime<Utc>,
    pub tries: u16,
    pub error: String,
    pub stack_trace: String,
}

#[derive(Debug)]
pub enum AggsState {
    Ok,
    Error(AggsError),
}

#[derive(Debug)]
pub struct AggsPipeContext {
    pub id: String,
    pub utc: DateTime<Utc>,
    pub click: ClickStreamItem,
    pub data: HashMap<&'static str, AggsPipeData>,
    pub state: AggsState,
}

impl AggsPipeContext {
    pub fn new(click: ClickStreamItem) -> Self {
        Self {
            id: Ulid::new().to_string(),
            utc: Utc::now(),
            click: click,
            data: HashMap::new(),
            state: AggsState::Ok,
        }
    }
}

impl AggsPipeContext {
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
        let _ = &self.data.insert(bool_key, AggsPipeData::Bool(value));
    }

    ///
    /// Adds a string value to the context's data
    ///
    pub fn add_string(&mut self, bool_key: &'static str, value: &'static str) {
        let _ = &self.data.insert(bool_key, AggsPipeData::String(value));
    }

    ///
    /// Adds a num value to the context's data
    ///
    pub fn add_num(&mut self, bool_key: &'static str, value: f64) {
        let _ = &self.data.insert(bool_key, AggsPipeData::Number(value));
    }
}

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct ClickStreamItem {
    pub id: String,
    pub owner_id: Option<String>,
    pub creator_id: Option<String>,
    pub route_id: Option<String>,
    pub workspace_id: Option<String>,
    pub created: DateTime<Utc>,
    pub dest: Option<String>,
    pub ip: Option<String>,
    pub continent: Option<String>,
    pub country: Option<String>,
    pub location: Option<String>,
    pub os_family: Option<String>,
    pub os_version: Option<String>,
    pub user_agent_family: Option<String>,
    pub user_agent_version: Option<String>,
    pub device_brand: Option<String>,
    pub device_family: Option<String>,
    pub device_model: Option<String>,
    pub session_first: Option<DateTime<Utc>>,
    pub session_clicks: Option<u128>,
    pub is_unique: bool,
    pub is_bot: bool,
}
