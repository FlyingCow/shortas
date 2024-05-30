use std::{net::IpAddr, sync::Arc};

use maxminddb::{geoip2, MaxMindDBError, Reader};

use crate::core::location_detect::{BaseLocationDetector, Country};

#[derive(Clone, Debug)]
pub struct GeoIPLocationDetector {
    reader: Arc<Reader<Vec<u8>>>,
}

impl GeoIPLocationDetector {
    pub fn new(path: &str) -> Self {
        println!("  mmdb -> {}", path);

        let reader = Reader::open_readfile(path).unwrap();

        Self {
            reader: Arc::new(reader),
        }
    }
}

impl BaseLocationDetector for GeoIPLocationDetector {
    fn detect_country(&self, &ip_addr: &IpAddr) -> Option<Country> {

        let country_detect_result: Result<geoip2::Country, MaxMindDBError> = self.reader.lookup(ip_addr);

        if country_detect_result.is_err(){
            return None;
        }

        let country = country_detect_result.unwrap();

        match country.country {
            Some(country) => Some(Country{ iso_code: country.iso_code.unwrap_or_default().to_ascii_lowercase() }),
            None => None,
        }
    }
}
