use anyhow::Result;
use tracing::info;

use crate::{
    core::{
        click_aggs_register::BaseClickAggsRegistrar, hit_stream::BaseHitStream,
        location_detect::BaseLocationDetector, user_agent_detect::BaseUserAgentDetector,
        BaseUserSettingsManager, BaseUserSettingsStore,
    },
    settings::Settings,
    tracking_pipe::{
        default_tracking_pipe::DefaultTrackingPipe, tracking_module::BaseTrackingModule,
    },
};

#[derive(Clone)]
pub struct AppBuilder {
    pub(super) settings: Settings,
    pub(super) user_settings_store: Option<Box<dyn BaseUserSettingsStore + Send + Sync + 'static>>,
    pub(super) user_settings_manager:
        Option<Box<dyn BaseUserSettingsManager + Send + Sync + 'static>>,
    pub(super) user_agent_detector: Option<Box<dyn BaseUserAgentDetector + Send + Sync + 'static>>,

    pub(super) location_detector: Option<Box<dyn BaseLocationDetector + Send + Sync + 'static>>,

    pub(super) hit_stream: Option<Box<dyn BaseHitStream + Send + Sync + 'static>>,
    pub(super) click_aggs_registrar:
        Option<Box<dyn BaseClickAggsRegistrar + Send + Sync + 'static>>,

    pub(super) modules: Vec<Box<dyn BaseTrackingModule + Send + Sync + 'static>>,
}

impl AppBuilder {
    pub fn new(settings: Settings) -> Self {
        Self {
            settings,
            user_settings_store: None,
            user_settings_manager: None,
            user_agent_detector: None,
            location_detector: None,
            hit_stream: None,
            click_aggs_registrar: None,
            modules: vec![],
        }
    }

    pub fn build(&self) -> Result<DefaultTrackingPipe> {
        info!("{}", "BUILDING");

        let hit_stream = &self.hit_stream.clone().unwrap();

        let router = DefaultTrackingPipe::new(hit_stream.to_owned(), self.modules.clone());

        Ok(router)
    }
}
