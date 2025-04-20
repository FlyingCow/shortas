use std::collections::HashMap;
use std::net::IpAddr;

use chrono::DateTime;
use chrono::Utc;
use serde::{Deserialize, Serialize};

pub mod aggs;
pub mod hit_stream;
pub mod location;
pub mod pipe;
pub mod session;
pub mod tracking_pipe;
pub mod user_agent;
pub mod user_settings;

pub use hit_stream::HitStreamSource;
use session::Session;
use ulid::Ulid;
pub use user_agent::UserAgentDetector;
pub use user_settings::UserSettingsManager;
pub use user_settings::UserSettingsStore;

#[derive(Default, Clone, Debug)]
pub enum ActiveStatus {
    #[default]
    Active,
    Blocked,
}
#[derive(Clone, Debug, Serialize, Deserialize, Eq, Hash, PartialEq)]
pub struct Country {
    pub iso_code: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, Hash, PartialEq)]
pub struct Location {
    pub country: Option<Country>,
}

impl Default for Location {
    fn default() -> Self {
        Self { country: None }
    }
}

pub const SKIP_TRACKING: &'static str = "tracking";

#[derive(Default, Clone, Debug)]
pub struct UserSettings {
    pub user_id: String,
    pub user_email: String,
    pub api_key: Option<String>,
    pub active_status: ActiveStatus,
    pub debug: bool,
    pub overflow: bool,
    pub skip: Vec<String>,
    pub allowed_request_params: Vec<String>,
    pub allowed_destination_params: Vec<String>,
}

impl UserSettings {
    pub fn new(
        user_id: String,
        user_email: String,
        api_key: Option<String>,
        active_status: ActiveStatus,
        debug: bool,
        overflow: bool,
        skip: Vec<String>,
        allowed_request_params: Vec<String>,
        allowed_destination_params: Vec<String>,
    ) -> Self {
        Self {
            user_id,
            user_email,
            api_key,
            active_status,
            debug,
            overflow,
            skip,
            allowed_request_params,
            allowed_destination_params,
        }
    }
}

/// Describes the `Family` as well as the `Major`, `Minor`, `Patch`, and
/// `PatchMinor` versions of an `OS`
#[derive(Clone, Debug, Serialize, Deserialize, Eq, Hash, PartialEq)]
pub struct OS {
    pub family: String,
    pub major: Option<String>,
    pub minor: Option<String>,
    pub patch: Option<String>,
    pub patch_minor: Option<String>,
}

impl Default for OS {
    fn default() -> Self {
        Self {
            family: String::from("Other"),
            major: None,
            minor: None,
            patch: None,
            patch_minor: None,
        }
    }
}

/// Describes the `Family` as well as the `Major`, `Minor`, and `Patch` versions
/// of a `UserAgent` client
#[derive(Clone, Debug, Deserialize, Serialize, Eq, Hash, PartialEq)]
pub struct UserAgent {
    pub family: String,
    pub major: Option<String>,
    pub minor: Option<String>,
    pub patch: Option<String>,
}

impl Default for UserAgent {
    fn default() -> Self {
        Self {
            family: String::from("Other"),
            major: None,
            minor: None,
            patch: None,
        }
    }
}

/// Describes the `Family`, `Brand` and `Model` of a `Device`
#[derive(Clone, Debug, Serialize, Deserialize, Eq, Hash, PartialEq)]
pub struct Device {
    pub family: String,
    pub brand: Option<String>,
    pub model: Option<String>,
}

impl Default for Device {
    fn default() -> Self {
        Self {
            family: String::from("Other"),
            brand: None,
            model: None,
        }
    }
}

/// Houses the `Device`, `OS`, and `UserAgent` structs, which each get parsed
/// out from a user agent string by a `UserAgentParser`.
#[derive(Clone, Debug, Serialize, Deserialize, Eq, Hash, PartialEq)]
pub struct Client {
    pub device: Device,
    pub os: OS,
    pub user_agent: UserAgent,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Click {
    pub dest: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Event {
    pub click: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum HitData {
    Click(Click),
    Event(Event),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HitRoute {
    pub id: Option<String>,
    pub owner_id: Option<String>,
    pub creator_id: Option<String>,
    pub workspace_id: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Hit {
    pub id: String,
    pub data: HitData,
    pub route: Option<HitRoute>,
    pub user_agent: Option<String>,
    pub ip: Option<IpAddr>,
    pub utc: DateTime<Utc>,
}

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
    pub stack_trace: String,
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
            spider: false,
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
