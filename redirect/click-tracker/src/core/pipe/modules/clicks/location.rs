use anyhow::Result;

use crate::{
    adapters::LocationDetectorType,
    core::{TrackingPipeContext, location::LocationDetector, tracking_pipe::TrackingModule},
};

#[derive(Clone)]
pub struct EnrichLocationModule {
    location_detector: LocationDetectorType,
}

#[async_trait::async_trait()]
impl TrackingModule for EnrichLocationModule {
    async fn execute(&mut self, context: &mut TrackingPipeContext) -> Result<()> {
        if let Some(ip) = context.hit.ip.clone() {
            let country = &self.location_detector.detect_country(&ip);

            if country.is_none() {
                return Ok(());
            }

            context.client_country = Some(country.clone().unwrap());
        }

        Ok(())
    }
}
impl EnrichLocationModule {
    pub fn new(location_detector: LocationDetectorType) -> Self {
        Self { location_detector }
    }
}
