use std::net::IpAddr;

use serde::{Deserialize, Serialize};

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

pub trait LocationDetector {
    fn detect_country(&self, ip_addr: &IpAddr) -> Option<Country>;
}
