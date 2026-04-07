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
                // Log module execution for debug routes
                if let Some(ref trace) = context.hit.trace {
                    tracing::warn!(
                        trace_id = %trace.trace_id,
                        route_id = %context.hit.route.as_ref().and_then(|r| r.id.as_ref()).map(|s| s.as_str()).unwrap_or(""),
                        service = "click-tracker",
                        step = "LocationModule",
                        country = "unknown",
                        "Debug trace: location module executed"
                    );
                }
                return Ok(());
            }

            context.client_country = Some(country.clone().unwrap());

            // Log module execution for debug routes
            if let Some(ref trace) = context.hit.trace {
                tracing::warn!(
                    trace_id = %trace.trace_id,
                    route_id = %context.hit.route.as_ref().and_then(|r| r.id.as_ref()).map(|s| s.as_str()).unwrap_or(""),
                    service = "click-tracker",
                    step = "LocationModule",
                    country = %context.client_country.as_ref().map(|c| c.iso_code.as_str()).unwrap_or(""),
                    "Debug trace: location module executed"
                );
            }
        }

        Ok(())
    }
}
impl EnrichLocationModule {
    pub fn new(location_detector: LocationDetectorType) -> Self {
        Self { location_detector }
    }
}
