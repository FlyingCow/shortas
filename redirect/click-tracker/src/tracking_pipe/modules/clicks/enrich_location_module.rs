use anyhow::Result;

use crate::{
    core::{location_detect::BaseLocationDetector, tracking_pipe::TrackingPipeContext},
    tracking_pipe::tracking_module::BaseTrackingModule,
};

#[derive(Clone)]
pub struct EnrichLocationModule {
    location_detector: Box<dyn BaseLocationDetector + Sync + Send + 'static>,
}

#[async_trait::async_trait()]
impl BaseTrackingModule for EnrichLocationModule {
    async fn execute(&mut self, context: &mut TrackingPipeContext) -> Result<()> {
        println!("{}", "Executing EnrichLocationModule");

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
    pub fn new(location_detector: Box<dyn BaseLocationDetector + Sync + Send + 'static>) -> Self {
        Self { location_detector }
    }
}
