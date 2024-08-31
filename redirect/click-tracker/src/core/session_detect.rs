
use std::net::IpAddr;

use chrono::{DateTime, Utc};
use dyn_clone::{clone_trait_object, DynClone};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, Eq, Hash, PartialEq)]
pub struct Country{
    pub iso_code: String, 
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, Hash, PartialEq)]
pub struct Session {
    pub first_click: DateTime<Utc>,
    pub count: u128
}

pub trait BaseSessionDetector: DynClone {
    fn detect_session(&self, route_id: &str, ip_addr: &IpAddr) -> Option<Session>;
}

clone_trait_object!(BaseSessionDetector);