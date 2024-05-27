use anyhow::Result;

use crate::{
    core::{
        base_location_detector::BaseLocationDetector,
        base_user_agent_detector::BaseUserAgentDetector, BaseCryptoManager, BaseCryptoStore,
        BaseRoutesManager, BaseRoutesStore, BaseUserSettingsManager, BaseUserSettingsStore,
    },
    flow_router::{
        base_flow_module::BaseFlowModule, base_host_extractor::BaseHostExtractor, base_ip_extractor::BaseIPExtractor, base_language_extractor::BaseLanguageExtractor, base_protocol_extractor::BaseProtocolExtractor, base_user_agent_string_extractor::BaseUserAgentStringExtractor, default_flow_router::DefaultFlowRouter
    },
    settings::Settings,
};
#[derive(Clone)]
pub struct AppBuilder {
    pub(super) settings: Settings,
    pub(super) routes_store: Option<Box<dyn BaseRoutesStore + 'static>>,
    pub(super) routes_manager: Option<Box<dyn BaseRoutesManager + 'static>>,
    pub(super) host_extractor: Option<Box<dyn BaseHostExtractor + 'static>>,
    pub(super) protocol_extractor: Option<Box<dyn BaseProtocolExtractor + 'static>>,
    pub(super) ip_extractor: Option<Box<dyn BaseIPExtractor + 'static>>,
    pub(super) user_agent_string_extractor: Option<Box<dyn BaseUserAgentStringExtractor + 'static>>,
    pub(super) language_extractor: Option<Box<dyn BaseLanguageExtractor + 'static>>,

    pub(super) crypto_store: Option<Box<dyn BaseCryptoStore + 'static>>,
    pub(super) crypto_manager: Option<Box<dyn BaseCryptoManager + 'static>>,
    pub(super) user_settings_store: Option<Box<dyn BaseUserSettingsStore + 'static>>,
    pub(super) user_settings_manager: Option<Box<dyn BaseUserSettingsManager + 'static>>,
    pub(super) user_agent_detector: Option<Box<dyn BaseUserAgentDetector + 'static>>,
    
    pub(super) location_detector: Option<Box<dyn BaseLocationDetector + 'static>>,
    pub(super) modules: Vec<Box<dyn BaseFlowModule + 'static>>,
}

impl AppBuilder {
    pub fn new(settings: Settings) -> Self {
        Self {
            settings,
            routes_store: None,
            routes_manager: None,
            crypto_store: None,
            crypto_manager: None,
            user_settings_store: None,
            user_settings_manager: None,
            host_extractor: None,
            ip_extractor: None,
            user_agent_string_extractor: None,
            language_extractor: None,
            protocol_extractor: None,
            user_agent_detector: None,
            location_detector: None,
            modules: vec![],
        }
    }

    pub fn build(&self) -> Result<DefaultFlowRouter> {
        println!("{}", "BUILDING");

        let router = DefaultFlowRouter::new(
            self.routes_manager.clone().unwrap(),
            self.host_extractor.clone().unwrap(),
            self.protocol_extractor.clone().unwrap(),
            self.ip_extractor.clone().unwrap(),
            self.user_agent_string_extractor.clone().unwrap(),
            self.language_extractor.clone().unwrap(),
            self.user_agent_detector.clone().unwrap(),
            self.location_detector.clone().unwrap(),
            self.modules.clone(),
        );

        Ok(router)
    }
}
