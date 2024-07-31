use std::net::IpAddr;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};


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
pub struct Hit {
    pub id: String,
    pub data: HitData,
    pub user_agent: Option<String>,
    pub ip: Option<IpAddr>,
    pub utc: DateTime<Utc>,
}
