use std::net::IpAddr;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Hit {
    pub id: String,
    pub utc: DateTime<Utc>,
    pub dest: Option<String>,
    pub user_agent: Option<String>,
    pub ip: Option<IpAddr>,
}

impl Hit {
    pub fn new(id: String, utc: DateTime<Utc>, dest: Option<String>, user_agent: Option<String>, ip: Option<IpAddr>) -> Self {
        Self {id,
            utc,
            dest,
            user_agent,
            ip
        }
    }
}
