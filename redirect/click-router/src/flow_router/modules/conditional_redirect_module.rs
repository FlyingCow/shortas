use anyhow::{Ok, Result};
use http::Method;

use crate::{
    core::{
        base_flow_router::{FlowRouterContext, FlowStep},
        BaseUserSettingsManager,
    },
    flow_router::{
        base_flow_module::{BaseFlowModule, FlowStepContinuation},
        default_flow_router::DefaultFlowRouter,
    },
    model::user_settings::SKIP_TRACKING,
};
const IS_CONDITIONAL: &'static str = "is_conditional";

#[derive(Clone)]
pub struct ConditionalRedirectModule {
    location_detector: Box<dyn BaseLocationDetector>,
}

impl ConditionalRedirectModule {
    pub fn new(location_detector: Box<dyn BaseLocationDetector>) -> Self {
        Self{
            location_detector
        }
    }
}

#[async_trait::async_trait(?Send)]
impl BaseFlowModule for ConditionalRedirectModule {
    async fn handle_start(
        &self,
        context: &mut FlowRouterContext,
        _: &DefaultFlowRouter,
    ) -> Result<FlowStepContinuation> {

        return Ok(FlowStepContinuation::Continue);
    }
}
