use std::net::IpAddr;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Hit {
    pub id: String,
    pub dest: Option<String>,
    pub user_agent: Option<String>,
    pub ip: Option<IpAddr>,
}

impl Hit {
    pub fn new(id: String, dest: Option<String>, user_agent: Option<String>, ip: Option<IpAddr>) -> Self {
        Self {id,
            dest,
            user_agent,
            ip
        }
    }
}
