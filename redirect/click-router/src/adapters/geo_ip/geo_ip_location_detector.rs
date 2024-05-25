use std::{net::IpAddr, sync::Arc};

use maxminddb::{geoip2, Reader};

use crate::core::base_location_detector::{BaseLocationDetector, Location};

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

impl BaseLocationDetector for GeoIPLocationDetector{
    fn detect_location<'a>(&self, &ip_addr: &'a IpAddr) -> Location<'a> {
        let _country: geoip2::Country = self.reader.lookup(ip_addr).unwrap();

        Location{
            ..Default::default()
        }
    }
}
