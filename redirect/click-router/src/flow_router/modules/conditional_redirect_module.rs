use anyhow::Result;

use crate::{core::{base_flow_router::FlowRouterContext, base_location_detector::BaseLocationDetector}, flow_router::{base_flow_module::{BaseFlowModule, FlowStepContinuation}, default_flow_router::DefaultFlowRouter}};

// const IS_CONDITIONAL: &'static str = "is_conditional";

#[derive(Clone)]
pub struct ConditionalRedirectModule {
    location_detector: Box<dyn BaseLocationDetector>,
}

impl ConditionalRedirectModule {
    pub fn new(location_detector: Box<dyn BaseLocationDetector>) -> Self {
        Self { location_detector }
    }
}

#[async_trait::async_trait(?Send)]
impl BaseFlowModule for ConditionalRedirectModule {
    async fn handle_start(
        &self,
        context: &mut FlowRouterContext,
        _: &DefaultFlowRouter,
    ) -> Result<FlowStepContinuation> {
        if context.client_ip.is_some() {
            let ip_addr = context.client_ip.as_ref().unwrap().address;

            let _location = &self.location_detector.detect_location(&ip_addr);
        }
        return Ok(FlowStepContinuation::Continue);
    }
}
