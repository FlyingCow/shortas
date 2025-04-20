use std::net::IpAddr;

use anyhow::Result;
use chrono::serde::ts_milliseconds;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, Eq, Hash, PartialEq)]
pub struct Session {
    #[serde(with = "ts_milliseconds")]
    pub first: DateTime<Utc>,
    pub count: u128,
}

#[async_trait::async_trait]
pub trait SessionDetector {
    async fn detect(
        &self,
        route_id: &str,
        ip_addr: &IpAddr,
        click_time: &DateTime<Utc>,
    ) -> Result<Session>;
}
