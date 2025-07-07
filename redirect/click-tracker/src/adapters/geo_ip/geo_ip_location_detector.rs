use maxminddb::{MaxMindDbError, Reader, geoip2};
use std::{net::IpAddr, sync::OnceLock};
use tracing::info;

use crate::core::{Country, location::LocationDetector};

use super::settings::GeoIP;

static MAXMIND_READER: OnceLock<Reader<Vec<u8>>> = OnceLock::new();

#[derive(Clone, Debug)]
pub struct GeoIPLocationDetector {
    reader: &'static Reader<Vec<u8>>,
}

impl GeoIPLocationDetector {
    pub fn new(settings: &GeoIP) -> Self {
        info!("  mmdb -> {}", &settings.mmdb);

        let reader = MAXMIND_READER.get_or_init(|| Reader::open_readfile(&settings.mmdb).unwrap());

        Self { reader }
    }
}

impl LocationDetector for GeoIPLocationDetector {
    fn detect_country(&self, &ip_addr: &IpAddr) -> Option<Country> {
        let country_detect_result: Result<Option<geoip2::Country>, MaxMindDbError> =
            self.reader.lookup(ip_addr);

        if country_detect_result.is_err() {
            return None;
        }

        let country_lookup_result = country_detect_result.unwrap();

        match country_lookup_result {
            Some(country) => match country.country {
                Some(country) => Some(Country {
                    iso_code: country.iso_code.unwrap_or_default().to_ascii_lowercase(),
                }),
                None => None,
            },
            None => None,
        }
    }
}
