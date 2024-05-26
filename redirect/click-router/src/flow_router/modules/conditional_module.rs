use anyhow::Result;
use chrono::{DateTime, prelude::*, Utc};
use crate::{
    core::{base_flow_router::FlowRouterContext, base_location_detector::BaseLocationDetector, base_user_agent_detector::BaseUserAgentDetector},
    flow_router::{
        base_flow_module::{BaseFlowModule, FlowStepContinuation},
        default_flow_router::DefaultFlowRouter,
    },
};

// const IS_CONDITIONAL: &'static str = "is_conditional";

#[derive(Clone)]
pub struct ConditionalModule {
    location_detector: Box<dyn BaseLocationDetector>,
}

impl ConditionalModule {
    pub fn new(location_detector: Box<dyn BaseLocationDetector>) -> Self {
        Self { location_detector }
    }
}

#[async_trait::async_trait(?Send)]
impl BaseFlowModule for ConditionalModule {
    async fn handle_start(
        &self,
        context: &mut FlowRouterContext,
        _: &DefaultFlowRouter,
    ) -> Result<FlowStepContinuation> {
        if context.client_ip.is_some() {
            let ip_addr = context.client_ip.as_ref().unwrap().address;

            let _location = &self.location_detector.detect_country(&ip_addr);
        }

        return Ok(FlowStepContinuation::Continue);
    }
}

pub struct ExpressionContext {
    user_agent_detector: Box<dyn BaseUserAgentDetector>
}

impl ExpressionContext {
    fn get_os(&self, flow_context: &mut FlowRouterContext)-> Result<String> {
        if flow_context.client_os.is_none(){
            //flow_context.client_os = &self.user_agent_detector.parse_os(&flow_context.)
        }
        Ok("Windows".into())
    }
    fn get_ua(&self) -> Result<String, std::io::Error> {
        Ok("Chrome".into())
    }
    fn get_day_of_month(&self) -> Result<u32, std::io::Error> {
        let now = Utc::now();
        Ok(now.day())
    }
    fn get_day_of_week(&self) -> Result<u32, std::io::Error> {
        let now = Utc::now();
        Ok(now.weekday().num_days_from_sunday())
    }
    fn get_month(&self) -> Result<u32, std::io::Error> {
        let now = Utc::now();
        Ok(now.month())
    }
    fn get_date(&self) -> Result<DateTime<Utc>, std::io::Error> {
        let now = Utc::now();
        Ok(now)
    }
}
