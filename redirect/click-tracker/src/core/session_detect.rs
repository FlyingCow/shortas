
use std::net::IpAddr;

use anyhow::Result;
use chrono::{DateTime, Utc};
use dyn_clone::{clone_trait_object, DynClone};
use serde::{Deserialize, Serialize};
use chrono::serde::ts_milliseconds;

#[derive(Clone, Debug, Serialize, Deserialize, Eq, Hash, PartialEq)]
pub struct Session {
    #[serde(with = "ts_milliseconds")]
    pub first: DateTime<Utc>,
    pub count: u128
}

pub trait BaseSessionDetector: DynClone {
    fn detect_session(&self, route_id: &str, ip_addr: &IpAddr, click_time: &DateTime<Utc>) -> Result<Session>;
}

clone_trait_object!(BaseSessionDetector);