use anyhow::Result;
use tracing::info;

use crate::{
    core::{
        location_detect::BaseLocationDetector, user_agent_detect::BaseUserAgentDetector,
        BaseUserSettingsManager, BaseUserSettingsStore,
    },
    settings::Settings,
    tracking_pipe::default_tracking_pipe::DefaultTrackingPipe,
};
#[derive(Clone)]
pub struct AppBuilder {
    pub(super) settings: Settings,
    pub(super) user_settings_store: Option<Box<dyn BaseUserSettingsStore + Send + Sync + 'static>>,
    pub(super) user_settings_manager:
        Option<Box<dyn BaseUserSettingsManager + Send + Sync + 'static>>,
    pub(super) user_agent_detector: Option<Box<dyn BaseUserAgentDetector + Send + Sync + 'static>>,

    pub(super) location_detector: Option<Box<dyn BaseLocationDetector + Send + Sync + 'static>>,
    //pub(super) modules: Vec<Box<dyn BaseFlowModule + Send + Sync + 'static>>,
}

impl AppBuilder {
    pub fn new(settings: Settings) -> Self {
        Self {
            settings,
            user_settings_store: None,
            user_settings_manager: None,
            user_agent_detector: None,
            location_detector: None,
            // modules: vec![],
        }
    }

    pub fn build(&self) -> Result<DefaultTrackingPipe> {
        info!("{}", "BUILDING");

        let router = DefaultTrackingPipe::new(
            self.user_agent_detector.clone().unwrap(),
            self.location_detector.clone().unwrap(),
        );

        Ok(router)
    }
}
