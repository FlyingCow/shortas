use tracing::info;

use crate::{adapters::geo_ip::geo_ip_location_detector::GeoIPLocationDetector, AppBuilder};

impl AppBuilder {
    pub fn with_geo_ip(&mut self) -> &mut Self {
        info!("{}", "WITH GEO-IP");

        let location_detector =
            Some(Box::new(GeoIPLocationDetector::new(&self.settings.geo_ip.mmdb)) as Box<_>);

        self.location_detector = location_detector;

        self
    }
}
