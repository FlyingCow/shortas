use std::{net::IpAddr, sync::Arc};

use maxminddb::{MaxMindDBError, Reader, geoip2};
use tracing::info;

use crate::core::{location::LocationDetector, Country};

#[derive(Clone, Debug)]
pub struct GeoIPLocationDetector {
    reader: Arc<Reader<Vec<u8>>>,
}

impl GeoIPLocationDetector {
    pub fn new(path: &str) -> Self {
        info!("  mmdb -> {}", path);

        let reader = Reader::open_readfile(path).unwrap();

        Self {
            reader: Arc::new(reader),
        }
    }
}

impl LocationDetector for GeoIPLocationDetector {
    fn detect_country(&self, &ip_addr: &IpAddr) -> Option<Country> {
        let country_detect_result: Result<geoip2::Country, MaxMindDBError> =
            self.reader.lookup(ip_addr);

        if country_detect_result.is_err() {
            return None;
        }

        let country = country_detect_result.unwrap();

        match country.country {
            Some(country) => Some(Country {
                iso_code: country.iso_code.unwrap_or_default().to_ascii_lowercase(),
            }),
            None => None,
        }
    }
}
