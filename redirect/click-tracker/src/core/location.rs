use std::net::IpAddr;

use super::Country;

pub trait LocationDetector {
    fn detect_country(&self, ip_addr: &IpAddr) -> Option<Country>;
}
